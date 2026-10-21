use dioxus::prelude::*;

#[component]
pub fn Card(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! {
        div {
            class: "rounded-lg border bg-card text-card-foreground shadow-sm {class.as_deref().unwrap_or(\"\")}",
            {children}
        }
    }
}

#[component]
pub fn CardHeader(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! {
        div {
            class: "flex flex-col space-y-1.5 p-6 {class.as_deref().unwrap_or(\"\")}",
            {children}
        }
    }
}

#[component]
pub fn CardTitle(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! {
        h3 {
            class: "text-2xl font-semibold leading-none tracking-tight {class.as_deref().unwrap_or(\"\")}",
            {children}
        }
    }
}

#[component]
pub fn CardDescription(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        p {
            class: "text-sm text-muted-foreground {class.as_deref().unwrap_or(\"\")}",
            {children}
        }
    }
}

#[component]
pub fn CardContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! {
        div {
            class: "p-6 pt-0 {class.as_deref().unwrap_or(\"\")}",
            {children}
        }
    }
}

#[component]
pub fn CardFooter(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! {
        div {
            class: "flex items-center p-6 pt-0 {class.as_deref().unwrap_or(\"\")}",
            {children}
        }
    }
}
