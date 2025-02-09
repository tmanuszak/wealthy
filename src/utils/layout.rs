use maud::Markup;

pub trait Page {
    fn page(self, title: impl std::fmt::Display) -> Markup;
}

impl<T> Page for T
where
    T: maud::Render,
{
    fn page(self, title: impl std::fmt::Display) -> Markup {
        maud::html! {
            (maud::DOCTYPE)
            html {
                head {
                    meta charset="UTF-8";
                    title { (title) }
                    link rel="preconnect" href="https://fonts.googleapis.com";
                    link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                    link href="https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&family=Source+Serif+4:ital,opsz,wght@0,8..60,200..900;1,8..60,200..900&display=swap" rel="stylesheet";
                    link href="https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&family=JetBrains+Mono:ital,wght@0,100..800;1,100..800&family=Source+Serif+4:ital,opsz,wght@0,8..60,200..900;1,8..60,200..900&display=swap" rel="stylesheet";
                    link rel="stylesheet" href="/static/css/style.css";
                    script src="/static/htmx.min.js" {}
                }
                body class="flex h-screen bg-stone-200" {
                    (crate::components::get_sidebar())
                    (self)
                }
            }
        }
    }
}
