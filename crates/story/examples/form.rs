use gpui::*;
use story::{Assets, FormStory};

pub struct Example {
    story: Entity<FormStory>,
}

impl Example {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let story = FormStory::view(window, cx);

        Self { story }
    }

    fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for Example {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().p_4().size_full().child(self.story.clone())
    }
}

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        story::init(cx);
        cx.activate(true);

        story::create_new_window("Form Example", Example::view, cx);
    });
}
