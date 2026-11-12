// Adds a link to the page in the other language of the book.
//
// The book is published as two mdBook books — the English one at the root of
// the site and the Russian one under `ru/` — so the counterpart of a page is
// the same file name in the other book.

(function () {
    "use strict";

    var path = window.location.pathname;
    var parts = path.split("/");
    var russian = parts.indexOf("ru");

    if (russian >= 0) {
        parts.splice(russian, 1);
    } else {
        var last = parts.length - 1;
        if (parts[last] === "") {
            parts[last] = "ru";
        } else {
            parts.splice(last, 0, "ru");
        }
    }

    var target = parts.join("/");
    if (target === path) {
        return;
    }

    var bar = document.querySelector(".menu-title");
    if (!bar) {
        return;
    }

    var link = document.createElement("a");
    link.className = "language-switch";
    link.href = target;
    link.textContent = russian >= 0 ? "English" : "Русский";
    link.title = russian >= 0 ? "Read this page in English" : "Читать страницу на английском";

    bar.parentNode.insertBefore(link, bar.nextSibling);
})();
