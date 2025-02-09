use maud::Markup;

pub fn get_sidebar() -> Markup {
    maud::html! {
        nav class="fixed left-0 h-full w-sidebar bg-sidebar flex flex-col items-center py-6 space-y-8" {
            // Logo
            div class="text-primary" {
                img src="/static/logo.svg" alt="Logo" class="w-8 h-8";
            }

            // Navigation Items
            div class="flex flex-col space-y-6" {
                a href="#" class="text-white hover:text-primary transition-colors" {
                    img src="/static/home.svg" alt="Homes" class="w-6 h-6";
                }
                a href="#" class="text-gray-500 hover:text-primary transition-colors" {
                    img src="/static/wallet.svg" alt="Wallet" class="w-6 h-6";
                }
            }
        }
    }
}
