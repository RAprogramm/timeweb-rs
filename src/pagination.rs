//! Offset pagination as a [`Stream`] over the generated list operations.

use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    task::{Context, Poll}
};

use futures_core::Stream;

use crate::apis::Error;

/// Streams every item of a paginated collection.
///
/// The closure receives `(limit, offset)` for each page and returns the
/// page's items; the stream yields them one by one, requesting the next
/// page on demand and finishing when a page comes back shorter than
/// `page_size`. The generated list operations take `Option<i32>` for both
/// parameters, so a closure usually just forwards them:
///
/// ```no_run
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// use futures_util::TryStreamExt;
/// use timeweb_rs::{apis::servers_api, paginate};
///
/// let config = timeweb_rs::authenticated("your-jwt-token");
/// let mut servers = paginate(100, |limit, offset| {
///     let config = &config;
///     async move {
///         let page = servers_api::get_servers(config, Some(limit), Some(offset)).await?;
///         Ok(page.servers)
///     }
/// });
/// while let Some(server) = servers.try_next().await? {
///     println!("{}", server.name);
/// }
/// # Ok(())
/// # }
/// ```
pub const fn paginate<T, E, F, Fut>(page_size: i32, fetch: F) -> PageStream<T, F, Fut>
where
    F: Fn(i32, i32) -> Fut + Unpin,
    Fut: Future<Output = Result<Vec<T>, Error<E>>>
{
    PageStream {
        fetch,
        page_size,
        offset: 0,
        buffer: VecDeque::new(),
        in_flight: None,
        done: false
    }
}

/// Stream created by [`paginate`]; yields `Result<T, Error<E>>` items.
pub struct PageStream<T, F, Fut> {
    fetch:     F,
    page_size: i32,
    offset:    i32,
    buffer:    VecDeque<T>,
    in_flight: Option<Pin<Box<Fut>>>,
    done:      bool
}

impl<T, E, F, Fut> Stream for PageStream<T, F, Fut>
where
    T: Unpin,
    F: Fn(i32, i32) -> Fut + Unpin,
    Fut: Future<Output = Result<Vec<T>, Error<E>>>
{
    type Item = Result<T, Error<E>>;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let stream = self.get_mut();
        loop {
            if let Some(item) = stream.buffer.pop_front() {
                return Poll::Ready(Some(Ok(item)));
            }
            if stream.done {
                return Poll::Ready(None);
            }
            let in_flight = stream
                .in_flight
                .get_or_insert_with(|| Box::pin((stream.fetch)(stream.page_size, stream.offset)));
            match in_flight.as_mut().poll(context) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(result) => {
                    stream.in_flight = None;
                    match result {
                        Err(error) => {
                            stream.done = true;
                            return Poll::Ready(Some(Err(error)));
                        }
                        Ok(page) => {
                            let received = i32::try_from(page.len()).unwrap_or(i32::MAX);
                            stream.offset = stream.offset.saturating_add(received);
                            if received < stream.page_size {
                                stream.done = true;
                            }
                            stream.buffer.extend(page);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use futures_util::{StreamExt, TryStreamExt};

    use super::paginate;
    use crate::apis::{Error, ResponseContent};

    fn not_found() -> Error<()> {
        Error::ResponseError(ResponseContent {
            status:  reqwest::StatusCode::NOT_FOUND,
            content: String::new(),
            entity:  None
        })
    }

    #[tokio::test]
    async fn yields_every_item_across_pages_in_order() {
        let requests = Mutex::new(Vec::new());
        let items: Vec<i32> = paginate(3, |limit, offset| {
            requests.lock().expect("lock").push((limit, offset));
            let page: Vec<i32> = (offset..(offset + limit).min(8)).collect();
            async move { Ok::<_, Error<()>>(page) }
        })
        .try_collect()
        .await
        .expect("all pages succeed");

        assert_eq!(items, (0..8).collect::<Vec<i32>>());
        assert_eq!(
            *requests.lock().expect("lock"),
            vec![(3, 0), (3, 3), (3, 6)]
        );
    }

    #[tokio::test]
    async fn a_full_final_page_triggers_one_trailing_empty_request() {
        let requests = Mutex::new(0u32);
        let items: Vec<u32> = paginate(2, |_, offset| {
            *requests.lock().expect("lock") += 1;
            let page: Vec<u32> = if offset < 4 { vec![1, 2] } else { Vec::new() };
            async move { Ok::<_, Error<()>>(page) }
        })
        .try_collect()
        .await
        .expect("all pages succeed");

        assert_eq!(items.len(), 4);
        assert_eq!(*requests.lock().expect("lock"), 3);
    }

    #[tokio::test]
    async fn an_empty_collection_finishes_immediately() {
        let items: Vec<u32> = paginate(10, |_, _| async { Ok::<Vec<u32>, Error<()>>(Vec::new()) })
            .try_collect()
            .await
            .expect("empty collection succeeds");
        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn an_error_is_yielded_once_and_ends_the_stream() {
        let mut stream = paginate(2, |_, offset| async move {
            if offset == 0 {
                Ok::<Vec<u32>, Error<()>>(vec![1, 2])
            } else {
                Err(not_found())
            }
        });

        assert_eq!(
            stream
                .next()
                .await
                .expect("first item")
                .expect("first page item"),
            1
        );
        assert_eq!(
            stream
                .next()
                .await
                .expect("second item")
                .expect("first page item"),
            2
        );
        assert!(
            stream
                .next()
                .await
                .expect("third poll yields the error")
                .is_err()
        );
        assert!(stream.next().await.is_none());
    }
}
