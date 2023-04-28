use cursive::{
    default,
    view::Nameable,
    views::{Dialog, EditView},
};

fn main() {
    let mut app = cursive::default();

    app.set_user_data(AppContext::default());
    // TODO:
    app.add_layer(
        Dialog::around(EditView::default().with_name("Editor")).title("Type and use buttons"),
    );
}

#[derive(Default)]
struct AppContext {
    clipboard: String,
    history: Vec<Box<dyn Command>>,
}

trait Command {
    fn execute(&mut self, app: &mut cursive::Cursive) -> bool;
    fn undo(&mut self, app: &mut cursive::Cursive);
}

#[derive(Default)]
struct CopyCommand;

impl Command for CopyCommand {
    fn execute(&mut self, app: &mut cursive::Cursive) -> bool {
        let editor = app.find_name::<EditView>("Editor").unwrap();
        let mut context = app.take_user_data::<AppContext>().unwrap();

        context.clipboard = editor.get_content().to_string();

        app.set_user_data(context);

        false
    }

    fn undo(&mut self, _app: &mut cursive::Cursive) {}
}

#[derive(Default)]
struct CutCommand {
    backup: String,
}

impl Command for CutCommand {
    fn execute(&mut self, app: &mut cursive::Cursive) -> bool {
        let mut editor = app.find_name::<EditView>("Editor").unwrap();

        app.with_user_data(|context: &mut AppContext| {
            self.backup = editor.get_content().to_string();
            context.clipboard = self.backup.clone();
            editor.set_content("".to_string());
        });

        true
    }

    fn undo(&mut self, app: &mut cursive::Cursive) {
        let mut editor = app.find_name::<EditView>("Editor").unwrap();
        editor.set_content(&self.backup);
    }
}

#[derive(Default)]
struct PasteCommand {
    backup: String,
}

impl Command for PasteCommand {
    fn execute(&mut self, app: &mut cursive::Cursive) -> bool {
        let mut editor = app.find_name::<EditView>("Editor").unwrap();

        app.with_user_data(|context: &mut AppContext| {
            self.backup = editor.get_content().to_string();
            editor.set_content(context.clipboard.clone());
        });

        true
    }

    fn undo(&mut self, app: &mut cursive::Cursive) {
        let mut editor = app.find_name::<EditView>("Editor").unwrap();
        editor.set_content(&self.backup);
    }
}
