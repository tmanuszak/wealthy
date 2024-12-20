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
                    link rel="stylesheet" href="/static/css/style.css";
                }
                body {
                    (self)
                }
            }
        }
    }
}
