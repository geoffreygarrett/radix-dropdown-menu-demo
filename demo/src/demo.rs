
use leptos::prelude::*;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuLabel, DropdownMenuPortal, DropdownMenuSeparator,
    DropdownMenuShortcut, DropdownMenuSub, DropdownMenuSubContent,
    DropdownMenuSubTrigger, DropdownMenuTrigger,
};

#[component]
#[allow(non_snake_case)]
pub fn DropdownMenuDemo() -> impl IntoView {
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger as_child=true>
                <Button variant=ButtonVariant::Outline>"Open"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent class="w-56" {..} class=("w-56", true)>
                <DropdownMenuLabel>"My Account"</DropdownMenuLabel>
                <DropdownMenuSeparator />
                <DropdownMenuGroup>
                    <DropdownMenuItem>
                        "Profile" <DropdownMenuShortcut>"⇧⌘P"</DropdownMenuShortcut>
                    </DropdownMenuItem>
                    <DropdownMenuItem>
                        "Billing" <DropdownMenuShortcut>"⌘B"</DropdownMenuShortcut>
                    </DropdownMenuItem>
                    <DropdownMenuItem>
                        "Settings" <DropdownMenuShortcut>"⌘S"</DropdownMenuShortcut>
                    </DropdownMenuItem>
                    <DropdownMenuItem>
                        "Keyboard shortcuts" <DropdownMenuShortcut>"⌘K"</DropdownMenuShortcut>
                    </DropdownMenuItem>
                </DropdownMenuGroup>
                <DropdownMenuSeparator />
                <DropdownMenuGroup>
                    <DropdownMenuItem>"Team"</DropdownMenuItem>
                    <DropdownMenuSub>
                        <DropdownMenuSubTrigger>"Invite users"</DropdownMenuSubTrigger>
                        <DropdownMenuPortal>
                            <DropdownMenuSubContent>
                                <DropdownMenuItem>"Email"</DropdownMenuItem>
                                <DropdownMenuItem>"Message"</DropdownMenuItem>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem>"More..."</DropdownMenuItem>
                            </DropdownMenuSubContent>
                        </DropdownMenuPortal>
                    </DropdownMenuSub>
                    <DropdownMenuItem>
                        "New Team" <DropdownMenuShortcut>"⌘+T"</DropdownMenuShortcut>
                    </DropdownMenuItem>
                </DropdownMenuGroup>
                <DropdownMenuSeparator />
                <DropdownMenuItem>"GitHub"</DropdownMenuItem>
                <DropdownMenuItem>"Support"</DropdownMenuItem>
                <DropdownMenuItem>"API"</DropdownMenuItem>
                <DropdownMenuItem disabled=true>"API"</DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem>
                    "Log out" <DropdownMenuShortcut>"⇧⌘Q"</DropdownMenuShortcut>
                </DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}