use crate::model::buffer::Buffer;
use crate::model::cursor::Cursor;

pub struct PluginCommand {
    pub id: String,
    pub label: String,
    pub shortcut_hint: String,
}

pub trait EditorApi {
    fn active_buffer(&self) -> &Buffer;
    fn active_buffer_mut(&mut self) -> &mut Buffer;
    fn cursor_mut(&mut self) -> &mut Cursor;

    fn set_status_message(&mut self, message: &str);

    fn insert_text_at_cursor(&mut self, text: &str);

    fn clipboard_paste(&mut self) -> String;
    fn clipboard_copy(&mut self, text: &str);

    fn buffer_count(&self) -> usize;
    fn switch_to_buffer(&mut self, index: usize);
}

pub trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;

    fn on_load(&mut self, api: &mut dyn EditorApi);
    fn on_unload(&mut self, api: &mut dyn EditorApi);

    fn commands(&self) -> Vec<PluginCommand> {
        Vec::new()
    }

    fn on_command(&mut self, _api: &mut dyn EditorApi, _command_id: &str) {}

    fn on_buffer_opened(&mut self, _api: &mut dyn EditorApi, _buffer_index: usize) {}
    fn on_before_save(&mut self, _api: &mut dyn EditorApi, _buffer_index: usize) {}
    fn on_key_press(&mut self, _api: &mut dyn EditorApi, _key: i32) -> bool {
        false
    }
}
