// Chooses the language of the book and offers the link to the other one.
//
// The documentation is published twice: the English book at the root of the
// site and the Russian one under `ru/`. The counterpart of a page is therefore
// the same file name in the other book. The script runs in the head of every
// page, so a browser that prefers Russian is redirected before the English page
// is painted. A visitor who has already picked a language is never moved.

(function () {
    "use strict";

    var CHOSEN = "timeweb-rs:language";

    function counterpart(path) {
        var parts = path.split("/");
        var at = parts.indexOf("ru");

        if (at >= 0) {
            parts.splice(at, 1);
            return { path: parts.join("/"), language: "en" };
        }

        var last = parts.length - 1;
        if (parts[last] === "") {
            parts[last] = "ru";
        } else {
            parts.splice(last, 0, "ru");
        }
        return { path: parts.join("/"), language: "ru" };
    }

    function preferred() {
        var list = navigator.languages || [];
        if (list.length === 0) {
            list = [navigator.language || "en"];
        }
        for (var i = 0; i < list.length; i += 1) {
            if (String(list[i]).toLowerCase().indexOf("ru") === 0) {
                return "ru";
            }
        }
        return "en";
    }

    function read(key) {
        try {
            return window.localStorage.getItem(key);
        } catch (error) {
            return null;
        }
    }

    function store(key, value) {
        try {
            window.localStorage.setItem(key, value);
        } catch (error) {
            return;
        }
    }

    var here = window.location.pathname;
    var other = counterpart(here);
    var current = other.language === "ru" ? "en" : "ru";
    var missing = document.title.indexOf("Page not found") === 0;

    if (current === "en" && !missing && !read(CHOSEN) && preferred() === "ru") {
        window.location.replace(other.path + window.location.search + window.location.hash);
        return;
    }

    function addSwitch() {
        var title = document.querySelector(".menu-title");
        if (!title) {
            return;
        }

        var label = other.language === "ru" ? "Русский" : "English";
        var hint =
            other.language === "ru"
                ? "Читать страницу на русском"
                : "Read this page in English";

        var link = document.createElement("a");
        link.className = "language-switch";
        link.href = other.path;
        link.hreflang = other.language;
        link.lang = other.language;
        link.textContent = label;
        link.title = hint;
        link.setAttribute("aria-label", hint);
        link.addEventListener("click", function () {
            store(CHOSEN, other.language);
        });

        title.parentNode.insertBefore(link, title.nextSibling);
    }

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", addSwitch);
    } else {
        addSwitch();
    }
})();
