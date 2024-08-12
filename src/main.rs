mod editor;
use editor::Editor;


fn main(){
    // limpiar pantalla
    print!("\x1b[2J");
    let mut editor = Editor::default();
    editor.run();
    print!("\x1b[2J");
}
