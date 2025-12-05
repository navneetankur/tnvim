
use serde::Deserializer;
use crate::contseq::ContSeq;
use crate::TryFromValue;
use crate::Nvimapi;
use rmpv::Value;
use crate::Pairs;
use crate::error;
use serde::{Deserialize, Serialize};
type Boolean = bool;
type Integer = i64;
type Float = f64  ;
#[derive(Serialize, Deserialize, Debug)]
pub struct Buffer(pub Integer);
#[derive(Serialize, Deserialize, Debug)]
pub struct Window(pub Integer);
#[derive(Serialize, Deserialize, Debug)]
pub struct Tabpage(pub Integer);
type Array  = Vec<Value>;
type Dict   = Pairs<Value,Value>;
type Object = Value;
impl From<Buffer> for Value {
    fn from(that: Buffer) -> Self {
        Value::from(that.0)
    }
}
impl From<Window> for Value {
    fn from(that: Window) -> Self {
        Value::from(that.0)
    }
}
impl From<Tabpage> for Value {
    fn from(that: Tabpage) -> Self {
        Value::from(that.0)
    }
}
impl TryFromValue for Buffer {
    fn try_from_value(value: Value) -> error::Result<Self> {
        let Ok(rv) = Integer::try_from(value) else {
            return error::with_msg("expected integer.");
        };
        return Ok(Self(rv));
    }
}
impl TryFromValue for Window {
    fn try_from_value(value: Value) -> error::Result<Self> {
        let Ok(rv) = Integer::try_from(value) else {
            return error::with_msg("expected integer.");
        };
        return Ok(Self(rv));
    }
}
impl TryFromValue for Tabpage {
    fn try_from_value(value: Value) -> error::Result<Self> {
        let Ok(rv) = Integer::try_from(value) else {
            return error::with_msg("expected integer.");
        };
        return Ok(Self(rv));
    }
}

impl Nvimapi {
pub async fn get_autocmds<D: Deserialize<'static>>(&self, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_get_autocmds".into(), (opts, )).await
}
pub async fn get_autocmds_wv(&self, opts: Dict, ) -> error::Result<Array> {
	self.call_fn_wv("nvim_get_autocmds".into(), (opts, )).await
}
pub async fn create_autocmd(&self, event: impl Serialize, opts: impl Serialize, ) -> error::Result<Integer> {
	self.call_fn("nvim_create_autocmd".into(), (event, opts, )).await
}
pub async fn create_autocmd_wv(&self, event: Object, opts: Dict, ) -> error::Result<Integer> {
	self.call_fn_wv("nvim_create_autocmd".into(), (event, opts, )).await
}
pub async fn del_autocmd(&self, id: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_del_autocmd".into(), (id, )).await
}
pub async fn clear_autocmds(&self, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_clear_autocmds".into(), (opts, )).await
}
pub async fn clear_autocmds_wv(&self, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_clear_autocmds".into(), (opts, )).await
}
pub async fn create_augroup(&self, name: &str, opts: impl Serialize, ) -> error::Result<Integer> {
	self.call_fn("nvim_create_augroup".into(), (name, opts, )).await
}
pub async fn create_augroup_wv(&self, name: String, opts: Dict, ) -> error::Result<Integer> {
	self.call_fn_wv("nvim_create_augroup".into(), (name, opts, )).await
}
pub async fn del_augroup_by_id(&self, id: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_del_augroup_by_id".into(), (id, )).await
}
pub async fn del_augroup_by_name(&self, name: &str, ) -> error::Result<()> {
	self.call_fn("nvim_del_augroup_by_name".into(), (name, )).await
}
pub async fn exec_autocmds(&self, event: impl Serialize, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_exec_autocmds".into(), (event, opts, )).await
}
pub async fn exec_autocmds_wv(&self, event: Object, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_exec_autocmds".into(), (event, opts, )).await
}
pub async fn buf_line_count(&self, buffer: Buffer, ) -> error::Result<Integer> {
	self.call_fn("nvim_buf_line_count".into(), (buffer, )).await
}
pub async fn buf_attach(&self, buffer: Buffer, send_buffer: Boolean, opts: impl Serialize, ) -> error::Result<Boolean> {
	self.call_fn("nvim_buf_attach".into(), (buffer, send_buffer, opts, )).await
}
pub async fn buf_attach_wv(&self, buffer: Buffer, send_buffer: Boolean, opts: Dict, ) -> error::Result<Boolean> {
	self.call_fn_wv("nvim_buf_attach".into(), (buffer, send_buffer, opts, )).await
}
pub async fn buf_detach(&self, buffer: Buffer, ) -> error::Result<Boolean> {
	self.call_fn("nvim_buf_detach".into(), (buffer, )).await
}
pub async fn buf_get_lines(&self, buffer: Buffer, start: Integer, end: Integer, strict_indexing: Boolean, ) -> error::Result<Vec<String>> {
	self.call_fn("nvim_buf_get_lines".into(), (buffer, start, end, strict_indexing, )).await
}
pub async fn buf_set_lines(&self, buffer: Buffer, start: Integer, end: Integer, strict_indexing: Boolean, replacement: &[&str], ) -> error::Result<()> {
	self.call_fn("nvim_buf_set_lines".into(), (buffer, start, end, strict_indexing, replacement, )).await
}
pub async fn buf_set_text(&self, buffer: Buffer, start_row: Integer, start_col: Integer, end_row: Integer, end_col: Integer, replacement: &[&str], ) -> error::Result<()> {
	self.call_fn("nvim_buf_set_text".into(), (buffer, start_row, start_col, end_row, end_col, replacement, )).await
}
pub async fn buf_get_text(&self, buffer: Buffer, start_row: Integer, start_col: Integer, end_row: Integer, end_col: Integer, opts: impl Serialize, ) -> error::Result<Vec<String>> {
	self.call_fn("nvim_buf_get_text".into(), (buffer, start_row, start_col, end_row, end_col, opts, )).await
}
pub async fn buf_get_text_wv(&self, buffer: Buffer, start_row: Integer, start_col: Integer, end_row: Integer, end_col: Integer, opts: Dict, ) -> error::Result<Vec<String>> {
	self.call_fn_wv("nvim_buf_get_text".into(), (buffer, start_row, start_col, end_row, end_col, opts, )).await
}
pub async fn buf_get_offset(&self, buffer: Buffer, index: Integer, ) -> error::Result<Integer> {
	self.call_fn("nvim_buf_get_offset".into(), (buffer, index, )).await
}
pub async fn buf_get_var<D: Deserialize<'static>>(&self, buffer: Buffer, name: &str, ) -> error::Result<D> {
	self.call_fn("nvim_buf_get_var".into(), (buffer, name, )).await
}
pub async fn buf_get_var_wv(&self, buffer: Buffer, name: String, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_buf_get_var".into(), (buffer, name, )).await
}
pub async fn buf_get_changedtick(&self, buffer: Buffer, ) -> error::Result<Integer> {
	self.call_fn("nvim_buf_get_changedtick".into(), (buffer, )).await
}
pub async fn buf_get_keymap<D: Deserialize<'static>>(&self, buffer: Buffer, mode: &str, ) -> error::Result<D> {
	self.call_fn("nvim_buf_get_keymap".into(), (buffer, mode, )).await
}
pub async fn buf_get_keymap_wv(&self, buffer: Buffer, mode: String, ) -> error::Result<Vec<Dict>> {
	self.call_fn_wv("nvim_buf_get_keymap".into(), (buffer, mode, )).await
}
pub async fn buf_set_keymap(&self, buffer: Buffer, mode: &str, lhs: &str, rhs: &str, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_buf_set_keymap".into(), (buffer, mode, lhs, rhs, opts, )).await
}
pub async fn buf_set_keymap_wv(&self, buffer: Buffer, mode: String, lhs: String, rhs: String, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_buf_set_keymap".into(), (buffer, mode, lhs, rhs, opts, )).await
}
pub async fn buf_del_keymap(&self, buffer: Buffer, mode: &str, lhs: &str, ) -> error::Result<()> {
	self.call_fn("nvim_buf_del_keymap".into(), (buffer, mode, lhs, )).await
}
pub async fn buf_set_var(&self, buffer: Buffer, name: &str, value: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_buf_set_var".into(), (buffer, name, value, )).await
}
pub async fn buf_set_var_wv(&self, buffer: Buffer, name: String, value: Object, ) -> error::Result<()> {
	self.call_fn_wv("nvim_buf_set_var".into(), (buffer, name, value, )).await
}
pub async fn buf_del_var(&self, buffer: Buffer, name: &str, ) -> error::Result<()> {
	self.call_fn("nvim_buf_del_var".into(), (buffer, name, )).await
}
pub async fn buf_get_name(&self, buffer: Buffer, ) -> error::Result<String> {
	self.call_fn("nvim_buf_get_name".into(), (buffer, )).await
}
pub async fn buf_set_name(&self, buffer: Buffer, name: &str, ) -> error::Result<()> {
	self.call_fn("nvim_buf_set_name".into(), (buffer, name, )).await
}
pub async fn buf_is_loaded(&self, buffer: Buffer, ) -> error::Result<Boolean> {
	self.call_fn("nvim_buf_is_loaded".into(), (buffer, )).await
}
pub async fn buf_delete(&self, buffer: Buffer, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_buf_delete".into(), (buffer, opts, )).await
}
pub async fn buf_delete_wv(&self, buffer: Buffer, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_buf_delete".into(), (buffer, opts, )).await
}
pub async fn buf_is_valid(&self, buffer: Buffer, ) -> error::Result<Boolean> {
	self.call_fn("nvim_buf_is_valid".into(), (buffer, )).await
}
pub async fn buf_del_mark(&self, buffer: Buffer, name: &str, ) -> error::Result<Boolean> {
	self.call_fn("nvim_buf_del_mark".into(), (buffer, name, )).await
}
pub async fn buf_set_mark(&self, buffer: Buffer, name: &str, line: Integer, col: Integer, opts: impl Serialize, ) -> error::Result<Boolean> {
	self.call_fn("nvim_buf_set_mark".into(), (buffer, name, line, col, opts, )).await
}
pub async fn buf_set_mark_wv(&self, buffer: Buffer, name: String, line: Integer, col: Integer, opts: Dict, ) -> error::Result<Boolean> {
	self.call_fn_wv("nvim_buf_set_mark".into(), (buffer, name, line, col, opts, )).await
}
pub async fn buf_get_mark(&self, buffer: Buffer, name: &str, ) -> error::Result<Vec<Integer>> {
	self.call_fn("nvim_buf_get_mark".into(), (buffer, name, )).await
}
pub async fn parse_cmd<D: Deserialize<'static>>(&self, str: &str, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_parse_cmd".into(), (str, opts, )).await
}
pub async fn parse_cmd_wv(&self, str: String, opts: Dict, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_parse_cmd".into(), (str, opts, )).await
}
pub async fn cmd(&self, cmd: impl Serialize, opts: impl Serialize, ) -> error::Result<String> {
	self.call_fn("nvim_cmd".into(), (cmd, opts, )).await
}
pub async fn cmd_wv(&self, cmd: Dict, opts: Dict, ) -> error::Result<String> {
	self.call_fn_wv("nvim_cmd".into(), (cmd, opts, )).await
}
pub async fn create_user_command(&self, name: &str, command: impl Serialize, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_create_user_command".into(), (name, command, opts, )).await
}
pub async fn create_user_command_wv(&self, name: String, command: Object, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_create_user_command".into(), (name, command, opts, )).await
}
pub async fn del_user_command(&self, name: &str, ) -> error::Result<()> {
	self.call_fn("nvim_del_user_command".into(), (name, )).await
}
pub async fn buf_create_user_command(&self, buffer: Buffer, name: &str, command: impl Serialize, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_buf_create_user_command".into(), (buffer, name, command, opts, )).await
}
pub async fn buf_create_user_command_wv(&self, buffer: Buffer, name: String, command: Object, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_buf_create_user_command".into(), (buffer, name, command, opts, )).await
}
pub async fn buf_del_user_command(&self, buffer: Buffer, name: &str, ) -> error::Result<()> {
	self.call_fn("nvim_buf_del_user_command".into(), (buffer, name, )).await
}
pub async fn get_commands<D: Deserialize<'static>>(&self, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_get_commands".into(), (opts, )).await
}
pub async fn get_commands_wv(&self, opts: Dict, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_get_commands".into(), (opts, )).await
}
pub async fn buf_get_commands<D: Deserialize<'static>>(&self, buffer: Buffer, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_buf_get_commands".into(), (buffer, opts, )).await
}
pub async fn buf_get_commands_wv(&self, buffer: Buffer, opts: Dict, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_buf_get_commands".into(), (buffer, opts, )).await
}
pub async fn create_namespace(&self, name: &str, ) -> error::Result<Integer> {
	self.call_fn("nvim_create_namespace".into(), (name, )).await
}
pub async fn get_namespaces<D: Deserialize<'static>>(&self, ) -> error::Result<D> {
	self.call_fn("nvim_get_namespaces".into(), [();0]).await
}
pub async fn get_namespaces_wv(&self, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_get_namespaces".into(), [();0]).await
}
pub async fn buf_get_extmark_by_id(&self, buffer: Buffer, ns_id: Integer, id: Integer, opts: impl Serialize, ) -> error::Result<Vec<Integer>> {
	self.call_fn("nvim_buf_get_extmark_by_id".into(), (buffer, ns_id, id, opts, )).await
}
pub async fn buf_get_extmark_by_id_wv(&self, buffer: Buffer, ns_id: Integer, id: Integer, opts: Dict, ) -> error::Result<Vec<Integer>> {
	self.call_fn_wv("nvim_buf_get_extmark_by_id".into(), (buffer, ns_id, id, opts, )).await
}
pub async fn buf_get_extmarks<D: Deserialize<'static>>(&self, buffer: Buffer, ns_id: Integer, start: impl Serialize, end: impl Serialize, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_buf_get_extmarks".into(), (buffer, ns_id, start, end, opts, )).await
}
pub async fn buf_get_extmarks_wv(&self, buffer: Buffer, ns_id: Integer, start: Object, end: Object, opts: Dict, ) -> error::Result<Array> {
	self.call_fn_wv("nvim_buf_get_extmarks".into(), (buffer, ns_id, start, end, opts, )).await
}
pub async fn buf_set_extmark(&self, buffer: Buffer, ns_id: Integer, line: Integer, col: Integer, opts: impl Serialize, ) -> error::Result<Integer> {
	self.call_fn("nvim_buf_set_extmark".into(), (buffer, ns_id, line, col, opts, )).await
}
pub async fn buf_set_extmark_wv(&self, buffer: Buffer, ns_id: Integer, line: Integer, col: Integer, opts: Dict, ) -> error::Result<Integer> {
	self.call_fn_wv("nvim_buf_set_extmark".into(), (buffer, ns_id, line, col, opts, )).await
}
pub async fn buf_del_extmark(&self, buffer: Buffer, ns_id: Integer, id: Integer, ) -> error::Result<Boolean> {
	self.call_fn("nvim_buf_del_extmark".into(), (buffer, ns_id, id, )).await
}
pub async fn buf_clear_namespace(&self, buffer: Buffer, ns_id: Integer, line_start: Integer, line_end: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_buf_clear_namespace".into(), (buffer, ns_id, line_start, line_end, )).await
}
pub async fn set_decoration_provider(&self, ns_id: Integer, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_set_decoration_provider".into(), (ns_id, opts, )).await
}
pub async fn set_decoration_provider_wv(&self, ns_id: Integer, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_set_decoration_provider".into(), (ns_id, opts, )).await
}
pub async fn get_option_value<D: Deserialize<'static>>(&self, name: &str, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_get_option_value".into(), (name, opts, )).await
}
pub async fn get_option_value_wv(&self, name: String, opts: Dict, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_get_option_value".into(), (name, opts, )).await
}
pub async fn set_option_value(&self, name: &str, value: impl Serialize, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_set_option_value".into(), (name, value, opts, )).await
}
pub async fn set_option_value_wv(&self, name: String, value: Object, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_set_option_value".into(), (name, value, opts, )).await
}
pub async fn get_all_options_info<D: Deserialize<'static>>(&self, ) -> error::Result<D> {
	self.call_fn("nvim_get_all_options_info".into(), [();0]).await
}
pub async fn get_all_options_info_wv(&self, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_get_all_options_info".into(), [();0]).await
}
pub async fn get_option_info2<D: Deserialize<'static>>(&self, name: &str, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_get_option_info2".into(), (name, opts, )).await
}
pub async fn get_option_info2_wv(&self, name: String, opts: Dict, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_get_option_info2".into(), (name, opts, )).await
}
pub async fn tabpage_list_wins(&self, tabpage: Tabpage, ) -> error::Result<Vec<Window>> {
	self.call_fn("nvim_tabpage_list_wins".into(), (tabpage, )).await
}
pub async fn tabpage_get_var<D: Deserialize<'static>>(&self, tabpage: Tabpage, name: &str, ) -> error::Result<D> {
	self.call_fn("nvim_tabpage_get_var".into(), (tabpage, name, )).await
}
pub async fn tabpage_get_var_wv(&self, tabpage: Tabpage, name: String, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_tabpage_get_var".into(), (tabpage, name, )).await
}
pub async fn tabpage_set_var(&self, tabpage: Tabpage, name: &str, value: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_tabpage_set_var".into(), (tabpage, name, value, )).await
}
pub async fn tabpage_set_var_wv(&self, tabpage: Tabpage, name: String, value: Object, ) -> error::Result<()> {
	self.call_fn_wv("nvim_tabpage_set_var".into(), (tabpage, name, value, )).await
}
pub async fn tabpage_del_var(&self, tabpage: Tabpage, name: &str, ) -> error::Result<()> {
	self.call_fn("nvim_tabpage_del_var".into(), (tabpage, name, )).await
}
pub async fn tabpage_get_win(&self, tabpage: Tabpage, ) -> error::Result<Window> {
	self.call_fn("nvim_tabpage_get_win".into(), (tabpage, )).await
}
pub async fn tabpage_set_win(&self, tabpage: Tabpage, win: Window, ) -> error::Result<()> {
	self.call_fn("nvim_tabpage_set_win".into(), (tabpage, win, )).await
}
pub async fn tabpage_get_number(&self, tabpage: Tabpage, ) -> error::Result<Integer> {
	self.call_fn("nvim_tabpage_get_number".into(), (tabpage, )).await
}
pub async fn tabpage_is_valid(&self, tabpage: Tabpage, ) -> error::Result<Boolean> {
	self.call_fn("nvim_tabpage_is_valid".into(), (tabpage, )).await
}
pub async fn ui_attach(&self, width: Integer, height: Integer, options: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_ui_attach".into(), (width, height, options, )).await
}
pub async fn ui_attach_wv(&self, width: Integer, height: Integer, options: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_ui_attach".into(), (width, height, options, )).await
}
pub async fn ui_set_focus(&self, gained: Boolean, ) -> error::Result<()> {
	self.call_fn("nvim_ui_set_focus".into(), (gained, )).await
}
pub async fn ui_detach(&self, ) -> error::Result<()> {
	self.call_fn("nvim_ui_detach".into(), [();0]).await
}
pub async fn ui_try_resize(&self, width: Integer, height: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_ui_try_resize".into(), (width, height, )).await
}
pub async fn ui_set_option(&self, name: &str, value: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_ui_set_option".into(), (name, value, )).await
}
pub async fn ui_set_option_wv(&self, name: String, value: Object, ) -> error::Result<()> {
	self.call_fn_wv("nvim_ui_set_option".into(), (name, value, )).await
}
pub async fn ui_try_resize_grid(&self, grid: Integer, width: Integer, height: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_ui_try_resize_grid".into(), (grid, width, height, )).await
}
pub async fn ui_pum_set_height(&self, height: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_ui_pum_set_height".into(), (height, )).await
}
pub async fn ui_pum_set_bounds(&self, width: Float, height: Float, row: Float, col: Float, ) -> error::Result<()> {
	self.call_fn("nvim_ui_pum_set_bounds".into(), (width, height, row, col, )).await
}
pub async fn ui_term_event(&self, event: &str, value: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_ui_term_event".into(), (event, value, )).await
}
pub async fn ui_term_event_wv(&self, event: String, value: Object, ) -> error::Result<()> {
	self.call_fn_wv("nvim_ui_term_event".into(), (event, value, )).await
}
pub async fn get_hl_id_by_name(&self, name: &str, ) -> error::Result<Integer> {
	self.call_fn("nvim_get_hl_id_by_name".into(), (name, )).await
}
pub async fn get_hl<D: Deserialize<'static>>(&self, ns_id: Integer, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_get_hl".into(), (ns_id, opts, )).await
}
pub async fn get_hl_wv(&self, ns_id: Integer, opts: Dict, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_get_hl".into(), (ns_id, opts, )).await
}
pub async fn set_hl(&self, ns_id: Integer, name: &str, val: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_set_hl".into(), (ns_id, name, val, )).await
}
pub async fn set_hl_wv(&self, ns_id: Integer, name: String, val: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_set_hl".into(), (ns_id, name, val, )).await
}
pub async fn get_hl_ns(&self, opts: impl Serialize, ) -> error::Result<Integer> {
	self.call_fn("nvim_get_hl_ns".into(), (opts, )).await
}
pub async fn get_hl_ns_wv(&self, opts: Dict, ) -> error::Result<Integer> {
	self.call_fn_wv("nvim_get_hl_ns".into(), (opts, )).await
}
pub async fn set_hl_ns(&self, ns_id: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_set_hl_ns".into(), (ns_id, )).await
}
pub async fn set_hl_ns_fast(&self, ns_id: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_set_hl_ns_fast".into(), (ns_id, )).await
}
pub async fn feedkeys(&self, keys: &str, mode: &str, escape_ks: Boolean, ) -> error::Result<()> {
	self.call_fn("nvim_feedkeys".into(), (keys, mode, escape_ks, )).await
}
pub async fn input(&self, keys: &str, ) -> error::Result<Integer> {
	self.call_fn("nvim_input".into(), (keys, )).await
}
pub async fn input_mouse(&self, button: &str, action: &str, modifier: &str, grid: Integer, row: Integer, col: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_input_mouse".into(), (button, action, modifier, grid, row, col, )).await
}
pub async fn replace_termcodes(&self, str: &str, from_part: Boolean, do_lt: Boolean, special: Boolean, ) -> error::Result<String> {
	self.call_fn("nvim_replace_termcodes".into(), (str, from_part, do_lt, special, )).await
}
pub async fn exec_lua<D: Deserialize<'static>>(&self, code: &str, args: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_exec_lua".into(), (code, args, )).await
}
pub async fn exec_lua_wv(&self, code: String, args: Array, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_exec_lua".into(), (code, args, )).await
}
pub async fn strwidth(&self, text: &str, ) -> error::Result<Integer> {
	self.call_fn("nvim_strwidth".into(), (text, )).await
}
pub async fn list_runtime_paths(&self, ) -> error::Result<Vec<String>> {
	self.call_fn("nvim_list_runtime_paths".into(), [();0]).await
}
pub async fn get_runtime_file(&self, name: &str, all: Boolean, ) -> error::Result<Vec<String>> {
	self.call_fn("nvim_get_runtime_file".into(), (name, all, )).await
}
pub async fn set_current_dir(&self, dir: &str, ) -> error::Result<()> {
	self.call_fn("nvim_set_current_dir".into(), (dir, )).await
}
pub async fn get_current_line(&self, ) -> error::Result<String> {
	self.call_fn("nvim_get_current_line".into(), [();0]).await
}
pub async fn set_current_line(&self, line: &str, ) -> error::Result<()> {
	self.call_fn("nvim_set_current_line".into(), (line, )).await
}
pub async fn del_current_line(&self, ) -> error::Result<()> {
	self.call_fn("nvim_del_current_line".into(), [();0]).await
}
pub async fn get_var<D: Deserialize<'static>>(&self, name: &str, ) -> error::Result<D> {
	self.call_fn("nvim_get_var".into(), (name, )).await
}
pub async fn get_var_wv(&self, name: String, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_get_var".into(), (name, )).await
}
pub async fn set_var(&self, name: &str, value: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_set_var".into(), (name, value, )).await
}
pub async fn set_var_wv(&self, name: String, value: Object, ) -> error::Result<()> {
	self.call_fn_wv("nvim_set_var".into(), (name, value, )).await
}
pub async fn del_var(&self, name: &str, ) -> error::Result<()> {
	self.call_fn("nvim_del_var".into(), (name, )).await
}
pub async fn get_vvar<D: Deserialize<'static>>(&self, name: &str, ) -> error::Result<D> {
	self.call_fn("nvim_get_vvar".into(), (name, )).await
}
pub async fn get_vvar_wv(&self, name: String, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_get_vvar".into(), (name, )).await
}
pub async fn set_vvar(&self, name: &str, value: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_set_vvar".into(), (name, value, )).await
}
pub async fn set_vvar_wv(&self, name: String, value: Object, ) -> error::Result<()> {
	self.call_fn_wv("nvim_set_vvar".into(), (name, value, )).await
}
pub async fn echo(&self, chunks: impl Serialize, history: Boolean, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_echo".into(), (chunks, history, opts, )).await
}
pub async fn echo_wv(&self, chunks: Array, history: Boolean, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_echo".into(), (chunks, history, opts, )).await
}
pub async fn list_bufs(&self, ) -> error::Result<Vec<Buffer>> {
	self.call_fn("nvim_list_bufs".into(), [();0]).await
}
pub async fn get_current_buf(&self, ) -> error::Result<Buffer> {
	self.call_fn("nvim_get_current_buf".into(), [();0]).await
}
pub async fn set_current_buf(&self, buffer: Buffer, ) -> error::Result<()> {
	self.call_fn("nvim_set_current_buf".into(), (buffer, )).await
}
pub async fn list_wins(&self, ) -> error::Result<Vec<Window>> {
	self.call_fn("nvim_list_wins".into(), [();0]).await
}
pub async fn get_current_win(&self, ) -> error::Result<Window> {
	self.call_fn("nvim_get_current_win".into(), [();0]).await
}
pub async fn set_current_win(&self, window: Window, ) -> error::Result<()> {
	self.call_fn("nvim_set_current_win".into(), (window, )).await
}
pub async fn create_buf(&self, listed: Boolean, scratch: Boolean, ) -> error::Result<Buffer> {
	self.call_fn("nvim_create_buf".into(), (listed, scratch, )).await
}
pub async fn open_term(&self, buffer: Buffer, opts: impl Serialize, ) -> error::Result<Integer> {
	self.call_fn("nvim_open_term".into(), (buffer, opts, )).await
}
pub async fn open_term_wv(&self, buffer: Buffer, opts: Dict, ) -> error::Result<Integer> {
	self.call_fn_wv("nvim_open_term".into(), (buffer, opts, )).await
}
pub async fn chan_send(&self, chan: Integer, data: &str, ) -> error::Result<()> {
	self.call_fn("nvim_chan_send".into(), (chan, data, )).await
}
pub async fn list_tabpages(&self, ) -> error::Result<Vec<Tabpage>> {
	self.call_fn("nvim_list_tabpages".into(), [();0]).await
}
pub async fn get_current_tabpage(&self, ) -> error::Result<Tabpage> {
	self.call_fn("nvim_get_current_tabpage".into(), [();0]).await
}
pub async fn set_current_tabpage(&self, tabpage: Tabpage, ) -> error::Result<()> {
	self.call_fn("nvim_set_current_tabpage".into(), (tabpage, )).await
}
pub async fn paste(&self, data: &str, crlf: Boolean, phase: Integer, ) -> error::Result<Boolean> {
	self.call_fn("nvim_paste".into(), (data, crlf, phase, )).await
}
pub async fn put(&self, lines: &[&str], type_: &str, after: Boolean, follow: Boolean, ) -> error::Result<()> {
	self.call_fn("nvim_put".into(), (lines, type_, after, follow, )).await
}
pub async fn get_color_by_name(&self, name: &str, ) -> error::Result<Integer> {
	self.call_fn("nvim_get_color_by_name".into(), (name, )).await
}
pub async fn get_color_map<D: Deserialize<'static>>(&self, ) -> error::Result<D> {
	self.call_fn("nvim_get_color_map".into(), [();0]).await
}
pub async fn get_color_map_wv(&self, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_get_color_map".into(), [();0]).await
}
pub async fn get_context<D: Deserialize<'static>>(&self, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_get_context".into(), (opts, )).await
}
pub async fn get_context_wv(&self, opts: Dict, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_get_context".into(), (opts, )).await
}
pub async fn load_context<D: Deserialize<'static>>(&self, dict: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_load_context".into(), (dict, )).await
}
pub async fn load_context_wv(&self, dict: Dict, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_load_context".into(), (dict, )).await
}
pub async fn get_mode<D: Deserialize<'static>>(&self, ) -> error::Result<D> {
	self.call_fn("nvim_get_mode".into(), [();0]).await
}
pub async fn get_mode_wv(&self, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_get_mode".into(), [();0]).await
}
pub async fn get_keymap<D: Deserialize<'static>>(&self, mode: &str, ) -> error::Result<D> {
	self.call_fn("nvim_get_keymap".into(), (mode, )).await
}
pub async fn get_keymap_wv(&self, mode: String, ) -> error::Result<Vec<Dict>> {
	self.call_fn_wv("nvim_get_keymap".into(), (mode, )).await
}
pub async fn set_keymap(&self, mode: &str, lhs: &str, rhs: &str, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_set_keymap".into(), (mode, lhs, rhs, opts, )).await
}
pub async fn set_keymap_wv(&self, mode: String, lhs: String, rhs: String, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_set_keymap".into(), (mode, lhs, rhs, opts, )).await
}
pub async fn del_keymap(&self, mode: &str, lhs: &str, ) -> error::Result<()> {
	self.call_fn("nvim_del_keymap".into(), (mode, lhs, )).await
}
pub async fn get_api_info<D: Deserialize<'static>>(&self, ) -> error::Result<D> {
	self.call_fn("nvim_get_api_info".into(), [();0]).await
}
pub async fn get_api_info_wv(&self, ) -> error::Result<Array> {
	self.call_fn_wv("nvim_get_api_info".into(), [();0]).await
}
pub async fn set_client_info(&self, name: &str, version: impl Serialize, type_: &str, methods: impl Serialize, attributes: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_set_client_info".into(), (name, version, type_, methods, attributes, )).await
}
pub async fn set_client_info_wv(&self, name: String, version: Dict, type_: String, methods: Dict, attributes: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_set_client_info".into(), (name, version, type_, methods, attributes, )).await
}
pub async fn get_chan_info<D: Deserialize<'static>>(&self, chan: Integer, ) -> error::Result<D> {
	self.call_fn("nvim_get_chan_info".into(), (chan, )).await
}
pub async fn get_chan_info_wv(&self, chan: Integer, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_get_chan_info".into(), (chan, )).await
}
pub async fn list_chans<D: Deserialize<'static>>(&self, ) -> error::Result<D> {
	self.call_fn("nvim_list_chans".into(), [();0]).await
}
pub async fn list_chans_wv(&self, ) -> error::Result<Array> {
	self.call_fn_wv("nvim_list_chans".into(), [();0]).await
}
pub async fn list_uis<D: Deserialize<'static>>(&self, ) -> error::Result<D> {
	self.call_fn("nvim_list_uis".into(), [();0]).await
}
pub async fn list_uis_wv(&self, ) -> error::Result<Array> {
	self.call_fn_wv("nvim_list_uis".into(), [();0]).await
}
pub async fn get_proc_children<D: Deserialize<'static>>(&self, pid: Integer, ) -> error::Result<D> {
	self.call_fn("nvim_get_proc_children".into(), (pid, )).await
}
pub async fn get_proc_children_wv(&self, pid: Integer, ) -> error::Result<Array> {
	self.call_fn_wv("nvim_get_proc_children".into(), (pid, )).await
}
pub async fn get_proc<D: Deserialize<'static>>(&self, pid: Integer, ) -> error::Result<D> {
	self.call_fn("nvim_get_proc".into(), (pid, )).await
}
pub async fn get_proc_wv(&self, pid: Integer, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_get_proc".into(), (pid, )).await
}
pub async fn select_popupmenu_item(&self, item: Integer, insert: Boolean, finish: Boolean, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_select_popupmenu_item".into(), (item, insert, finish, opts, )).await
}
pub async fn select_popupmenu_item_wv(&self, item: Integer, insert: Boolean, finish: Boolean, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_select_popupmenu_item".into(), (item, insert, finish, opts, )).await
}
pub async fn del_mark(&self, name: &str, ) -> error::Result<Boolean> {
	self.call_fn("nvim_del_mark".into(), (name, )).await
}
pub async fn get_mark<D: Deserialize<'static>>(&self, name: &str, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_get_mark".into(), (name, opts, )).await
}
pub async fn get_mark_wv(&self, name: String, opts: Dict, ) -> error::Result<Array> {
	self.call_fn_wv("nvim_get_mark".into(), (name, opts, )).await
}
pub async fn eval_statusline<D: Deserialize<'static>>(&self, str: &str, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_eval_statusline".into(), (str, opts, )).await
}
pub async fn eval_statusline_wv(&self, str: String, opts: Dict, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_eval_statusline".into(), (str, opts, )).await
}
pub async fn exec2<D: Deserialize<'static>>(&self, src: &str, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_exec2".into(), (src, opts, )).await
}
pub async fn exec2_wv(&self, src: String, opts: Dict, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_exec2".into(), (src, opts, )).await
}
pub async fn command(&self, command: &str, ) -> error::Result<()> {
	self.call_fn("nvim_command".into(), (command, )).await
}
pub async fn eval<D: Deserialize<'static>>(&self, expr: &str, ) -> error::Result<D> {
	self.call_fn("nvim_eval".into(), (expr, )).await
}
pub async fn eval_wv(&self, expr: String, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_eval".into(), (expr, )).await
}
pub async fn call_function<D: Deserialize<'static>>(&self, fn_: &str, args: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_call_function".into(), (fn_, args, )).await
}
pub async fn call_function_wv(&self, fn_: String, args: Array, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_call_function".into(), (fn_, args, )).await
}
pub async fn call_dict_function<D: Deserialize<'static>>(&self, dict: impl Serialize, fn_: &str, args: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_call_dict_function".into(), (dict, fn_, args, )).await
}
pub async fn call_dict_function_wv(&self, dict: Object, fn_: String, args: Array, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_call_dict_function".into(), (dict, fn_, args, )).await
}
pub async fn parse_expression<D: Deserialize<'static>>(&self, expr: &str, flags: &str, highlight: Boolean, ) -> error::Result<D> {
	self.call_fn("nvim_parse_expression".into(), (expr, flags, highlight, )).await
}
pub async fn parse_expression_wv(&self, expr: String, flags: String, highlight: Boolean, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_parse_expression".into(), (expr, flags, highlight, )).await
}
pub async fn open_win(&self, buffer: Buffer, enter: Boolean, config: impl Serialize, ) -> error::Result<Window> {
	self.call_fn("nvim_open_win".into(), (buffer, enter, config, )).await
}
pub async fn open_win_wv(&self, buffer: Buffer, enter: Boolean, config: Dict, ) -> error::Result<Window> {
	self.call_fn_wv("nvim_open_win".into(), (buffer, enter, config, )).await
}
pub async fn win_set_config(&self, window: Window, config: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_win_set_config".into(), (window, config, )).await
}
pub async fn win_set_config_wv(&self, window: Window, config: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_win_set_config".into(), (window, config, )).await
}
pub async fn win_get_config<D: Deserialize<'static>>(&self, window: Window, ) -> error::Result<D> {
	self.call_fn("nvim_win_get_config".into(), (window, )).await
}
pub async fn win_get_config_wv(&self, window: Window, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_win_get_config".into(), (window, )).await
}
pub async fn win_get_buf(&self, window: Window, ) -> error::Result<Buffer> {
	self.call_fn("nvim_win_get_buf".into(), (window, )).await
}
pub async fn win_set_buf(&self, window: Window, buffer: Buffer, ) -> error::Result<()> {
	self.call_fn("nvim_win_set_buf".into(), (window, buffer, )).await
}
pub async fn win_get_cursor(&self, window: Window, ) -> error::Result<Vec<Integer>> {
	self.call_fn("nvim_win_get_cursor".into(), (window, )).await
}
pub async fn win_set_cursor(&self, window: Window, pos: &[Integer], ) -> error::Result<()> {
	self.call_fn("nvim_win_set_cursor".into(), (window, pos, )).await
}
pub async fn win_get_height(&self, window: Window, ) -> error::Result<Integer> {
	self.call_fn("nvim_win_get_height".into(), (window, )).await
}
pub async fn win_set_height(&self, window: Window, height: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_win_set_height".into(), (window, height, )).await
}
pub async fn win_get_width(&self, window: Window, ) -> error::Result<Integer> {
	self.call_fn("nvim_win_get_width".into(), (window, )).await
}
pub async fn win_set_width(&self, window: Window, width: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_win_set_width".into(), (window, width, )).await
}
pub async fn win_get_var<D: Deserialize<'static>>(&self, window: Window, name: &str, ) -> error::Result<D> {
	self.call_fn("nvim_win_get_var".into(), (window, name, )).await
}
pub async fn win_get_var_wv(&self, window: Window, name: String, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_win_get_var".into(), (window, name, )).await
}
pub async fn win_set_var(&self, window: Window, name: &str, value: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_win_set_var".into(), (window, name, value, )).await
}
pub async fn win_set_var_wv(&self, window: Window, name: String, value: Object, ) -> error::Result<()> {
	self.call_fn_wv("nvim_win_set_var".into(), (window, name, value, )).await
}
pub async fn win_del_var(&self, window: Window, name: &str, ) -> error::Result<()> {
	self.call_fn("nvim_win_del_var".into(), (window, name, )).await
}
pub async fn win_get_position(&self, window: Window, ) -> error::Result<Vec<Integer>> {
	self.call_fn("nvim_win_get_position".into(), (window, )).await
}
pub async fn win_get_tabpage(&self, window: Window, ) -> error::Result<Tabpage> {
	self.call_fn("nvim_win_get_tabpage".into(), (window, )).await
}
pub async fn win_get_number(&self, window: Window, ) -> error::Result<Integer> {
	self.call_fn("nvim_win_get_number".into(), (window, )).await
}
pub async fn win_is_valid(&self, window: Window, ) -> error::Result<Boolean> {
	self.call_fn("nvim_win_is_valid".into(), (window, )).await
}
pub async fn win_hide(&self, window: Window, ) -> error::Result<()> {
	self.call_fn("nvim_win_hide".into(), (window, )).await
}
pub async fn win_close(&self, window: Window, force: Boolean, ) -> error::Result<()> {
	self.call_fn("nvim_win_close".into(), (window, force, )).await
}
pub async fn win_set_hl_ns(&self, window: Window, ns_id: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_win_set_hl_ns".into(), (window, ns_id, )).await
}
pub async fn win_text_height<D: Deserialize<'static>>(&self, window: Window, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_win_text_height".into(), (window, opts, )).await
}
pub async fn win_text_height_wv(&self, window: Window, opts: Dict, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_win_text_height".into(), (window, opts, )).await
}
}
#[derive(Deserialize, Debug)]
pub struct ModeInfoSet {
	pub enabled: Boolean,
	pub cursor_styles: Array,
}
#[derive(Deserialize, Debug)]
pub struct UpdateMenu {
}
#[derive(Deserialize, Debug)]
pub struct BusyStart {
}
#[derive(Deserialize, Debug)]
pub struct BusyStop {
}
#[derive(Deserialize, Debug)]
pub struct MouseOn {
}
#[derive(Deserialize, Debug)]
pub struct MouseOff {
}
#[derive(Deserialize, Debug)]
pub struct ModeChange {
	pub mode: String,
	pub mode_idx: Integer,
}
#[derive(Deserialize, Debug)]
pub struct Bell {
}
#[derive(Deserialize, Debug)]
pub struct VisualBell {
}
#[derive(Deserialize, Debug)]
pub struct Flush {
}
#[derive(Deserialize, Debug)]
pub struct Suspend {
}
#[derive(Deserialize, Debug)]
pub struct SetTitle {
	pub title: String,
}
#[derive(Deserialize, Debug)]
pub struct SetIcon {
	pub icon: String,
}
#[derive(Deserialize, Debug)]
pub struct Screenshot {
	pub path: String,
}
#[derive(Deserialize, Debug)]
pub struct OptionSet {
	pub name: String,
	pub value: Object,
}
#[derive(Deserialize, Debug)]
pub struct Chdir {
	pub path: String,
}
#[derive(Deserialize, Debug)]
pub struct UpdateFg {
	pub fg: Integer,
}
#[derive(Deserialize, Debug)]
pub struct UpdateBg {
	pub bg: Integer,
}
#[derive(Deserialize, Debug)]
pub struct UpdateSp {
	pub sp: Integer,
}
#[derive(Deserialize, Debug)]
pub struct Resize {
	pub width: Integer,
	pub height: Integer,
}
#[derive(Deserialize, Debug)]
pub struct Clear {
}
#[derive(Deserialize, Debug)]
pub struct EolClear {
}
#[derive(Deserialize, Debug)]
pub struct CursorGoto {
	pub row: Integer,
	pub col: Integer,
}
#[derive(Deserialize, Debug)]
pub struct HighlightSet {
	pub attrs: Dict,
}
#[derive(Deserialize, Debug)]
pub struct Put {
	pub str: String,
}
#[derive(Deserialize, Debug)]
pub struct SetScrollRegion {
	pub top: Integer,
	pub bot: Integer,
	pub left: Integer,
	pub right: Integer,
}
#[derive(Deserialize, Debug)]
pub struct Scroll {
	pub count: Integer,
}
#[derive(Deserialize, Debug)]
pub struct DefaultColorsSet {
	pub rgb_fg: Integer,
	pub rgb_bg: Integer,
	pub rgb_sp: Integer,
	pub cterm_fg: Integer,
	pub cterm_bg: Integer,
}
#[derive(Deserialize, Debug)]
pub struct HlAttrDefine {
	pub id: Integer,
	pub rgb_attrs: Dict,
	pub cterm_attrs: Dict,
	pub info: Array,
}
#[derive(Deserialize, Debug)]
pub struct HlGroupSet {
	pub name: String,
	pub id: Integer,
}
#[derive(Deserialize, Debug)]
pub struct GridResize {
	pub grid: Integer,
	pub width: Integer,
	pub height: Integer,
}
#[derive(Deserialize, Debug)]
pub struct GridClear {
	pub grid: Integer,
}
#[derive(Deserialize, Debug)]
pub struct GridCursorGoto {
	pub grid: Integer,
	pub row: Integer,
	pub col: Integer,
}
#[derive(Deserialize, Debug)]
pub struct GridLine {
	pub grid: Integer,
	pub row: Integer,
	pub col_start: Integer,
	pub data: Array,
	pub wrap: Boolean,
}
#[derive(Deserialize, Debug)]
pub struct GridScroll {
	pub grid: Integer,
	pub top: Integer,
	pub bot: Integer,
	pub left: Integer,
	pub right: Integer,
	pub rows: Integer,
	pub cols: Integer,
}
#[derive(Deserialize, Debug)]
pub struct GridDestroy {
	pub grid: Integer,
}
#[derive(Deserialize, Debug)]
pub struct WinPos {
	pub grid: Integer,
	pub win: Window,
	pub startrow: Integer,
	pub startcol: Integer,
	pub width: Integer,
	pub height: Integer,
}
#[derive(Deserialize, Debug)]
pub struct WinFloatPos {
	pub grid: Integer,
	pub win: Window,
	pub anchor: String,
	pub anchor_grid: Integer,
	pub anchor_row: Float,
	pub anchor_col: Float,
	pub mouse_enabled: Boolean,
	pub zindex: Integer,
}
#[derive(Deserialize, Debug)]
pub struct WinExternalPos {
	pub grid: Integer,
	pub win: Window,
}
#[derive(Deserialize, Debug)]
pub struct WinHide {
	pub grid: Integer,
}
#[derive(Deserialize, Debug)]
pub struct WinClose {
	pub grid: Integer,
}
#[derive(Deserialize, Debug)]
pub struct MsgSetPos {
	pub grid: Integer,
	pub row: Integer,
	pub scrolled: Boolean,
	pub sep_char: String,
}
#[derive(Deserialize, Debug)]
pub struct WinViewport {
	pub grid: Integer,
	pub win: Window,
	pub topline: Integer,
	pub botline: Integer,
	pub curline: Integer,
	pub curcol: Integer,
	pub line_count: Integer,
	pub scroll_delta: Integer,
}
#[derive(Deserialize, Debug)]
pub struct WinViewportMargins {
	pub grid: Integer,
	pub win: Window,
	pub top: Integer,
	pub bottom: Integer,
	pub left: Integer,
	pub right: Integer,
}
#[derive(Deserialize, Debug)]
pub struct WinExtmark {
	pub grid: Integer,
	pub win: Window,
	pub ns_id: Integer,
	pub mark_id: Integer,
	pub row: Integer,
	pub col: Integer,
}
#[derive(Deserialize, Debug)]
pub struct PopupmenuShow {
	pub items: Array,
	pub selected: Integer,
	pub row: Integer,
	pub col: Integer,
	pub grid: Integer,
}
#[derive(Deserialize, Debug)]
pub struct PopupmenuHide {
}
#[derive(Deserialize, Debug)]
pub struct PopupmenuSelect {
	pub selected: Integer,
}
#[derive(Deserialize, Debug)]
pub struct TablineUpdate {
	pub current: Tabpage,
	pub tabs: Array,
	pub current_buffer: Buffer,
	pub buffers: Array,
}
#[derive(Deserialize, Debug)]
pub struct CmdlineShow {
	pub content: Array,
	pub pos: Integer,
	pub firstc: String,
	pub prompt: String,
	pub indent: Integer,
	pub level: Integer,
	pub hl_id: Integer,
}
#[derive(Deserialize, Debug)]
pub struct CmdlinePos {
	pub pos: Integer,
	pub level: Integer,
}
#[derive(Deserialize, Debug)]
pub struct CmdlineSpecialChar {
	pub c: String,
	pub shift: Boolean,
	pub level: Integer,
}
#[derive(Deserialize, Debug)]
pub struct CmdlineHide {
	pub level: Integer,
	pub abort: Boolean,
}
#[derive(Deserialize, Debug)]
pub struct CmdlineBlockShow {
	pub lines: Array,
}
#[derive(Deserialize, Debug)]
pub struct CmdlineBlockAppend {
	pub lines: Array,
}
#[derive(Deserialize, Debug)]
pub struct CmdlineBlockHide {
}
#[derive(Deserialize, Debug)]
pub struct WildmenuShow {
	pub items: Array,
}
#[derive(Deserialize, Debug)]
pub struct WildmenuSelect {
	pub selected: Integer,
}
#[derive(Deserialize, Debug)]
pub struct WildmenuHide {
}
#[derive(Deserialize, Debug)]
pub struct MsgShow {
	pub kind: String,
	pub content: Array,
	pub replace_last: Boolean,
	pub history: Boolean,
}
#[derive(Deserialize, Debug)]
pub struct MsgClear {
}
#[derive(Deserialize, Debug)]
pub struct MsgShowcmd {
	pub content: Array,
}
#[derive(Deserialize, Debug)]
pub struct MsgShowmode {
	pub content: Array,
}
#[derive(Deserialize, Debug)]
pub struct MsgRuler {
	pub content: Array,
}
#[derive(Deserialize, Debug)]
pub struct MsgHistoryShow {
	pub entries: Array,
}
#[derive(Deserialize, Debug)]
pub struct MsgHistoryClear {
}
#[derive(Deserialize, Debug)]
pub struct ErrorExit {
	pub status: Integer,
}
#[derive(Debug)]
pub enum UiEvent {
	ModeInfoSet(ModeInfoSet),
	UpdateMenu(UpdateMenu),
	BusyStart(BusyStart),
	BusyStop(BusyStop),
	MouseOn(MouseOn),
	MouseOff(MouseOff),
	ModeChange(ModeChange),
	Bell(Bell),
	VisualBell(VisualBell),
	Flush(Flush),
	Suspend(Suspend),
	SetTitle(SetTitle),
	SetIcon(SetIcon),
	Screenshot(Screenshot),
	OptionSet(OptionSet),
	Chdir(Chdir),
	UpdateFg(UpdateFg),
	UpdateBg(UpdateBg),
	UpdateSp(UpdateSp),
	Resize(Resize),
	Clear(Clear),
	EolClear(EolClear),
	CursorGoto(CursorGoto),
	HighlightSet(HighlightSet),
	Put(Put),
	SetScrollRegion(SetScrollRegion),
	Scroll(Scroll),
	DefaultColorsSet(DefaultColorsSet),
	HlAttrDefine(HlAttrDefine),
	HlGroupSet(HlGroupSet),
	GridResize(GridResize),
	GridClear(GridClear),
	GridCursorGoto(GridCursorGoto),
	GridLine(GridLine),
	GridScroll(GridScroll),
	GridDestroy(GridDestroy),
	WinPos(WinPos),
	WinFloatPos(WinFloatPos),
	WinExternalPos(WinExternalPos),
	WinHide(WinHide),
	WinClose(WinClose),
	MsgSetPos(MsgSetPos),
	WinViewport(WinViewport),
	WinViewportMargins(WinViewportMargins),
	WinExtmark(WinExtmark),
	PopupmenuShow(PopupmenuShow),
	PopupmenuHide(PopupmenuHide),
	PopupmenuSelect(PopupmenuSelect),
	TablineUpdate(TablineUpdate),
	CmdlineShow(CmdlineShow),
	CmdlinePos(CmdlinePos),
	CmdlineSpecialChar(CmdlineSpecialChar),
	CmdlineHide(CmdlineHide),
	CmdlineBlockShow(CmdlineBlockShow),
	CmdlineBlockAppend(CmdlineBlockAppend),
	CmdlineBlockHide(CmdlineBlockHide),
	WildmenuShow(WildmenuShow),
	WildmenuSelect(WildmenuSelect),
	WildmenuHide(WildmenuHide),
	MsgShow(MsgShow),
	MsgClear(MsgClear),
	MsgShowcmd(MsgShowcmd),
	MsgShowmode(MsgShowmode),
	MsgRuler(MsgRuler),
	MsgHistoryShow(MsgHistoryShow),
	MsgHistoryClear(MsgHistoryClear),
	ErrorExit(ErrorExit),
	Unknown(String, Value),
}

impl<'de> Deserialize<'de> for UiEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        return deserializer.deserialize_seq(UVisitor);

        struct UVisitor;
        impl<'de> serde::de::Visitor<'de> for UVisitor {
            type Value = UiEvent;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("expecting seq for ui_event")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                use serde::de::{Error as DError};
                let msg = "missing element, expected 2 elements";
                let Some(event_name) = seq.next_element::<String>()? else {
                    return Err(DError::custom(msg));
                };
                match event_name.as_str() {
                "mode_info_set" => {
	let inner = ModeInfoSet::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::ModeInfoSet(inner));
},
"update_menu" => {
	let inner = UpdateMenu::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::UpdateMenu(inner));
},
"busy_start" => {
	let inner = BusyStart::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::BusyStart(inner));
},
"busy_stop" => {
	let inner = BusyStop::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::BusyStop(inner));
},
"mouse_on" => {
	let inner = MouseOn::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::MouseOn(inner));
},
"mouse_off" => {
	let inner = MouseOff::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::MouseOff(inner));
},
"mode_change" => {
	let inner = ModeChange::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::ModeChange(inner));
},
"bell" => {
	let inner = Bell::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::Bell(inner));
},
"visual_bell" => {
	let inner = VisualBell::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::VisualBell(inner));
},
"flush" => {
	let inner = Flush::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::Flush(inner));
},
"suspend" => {
	let inner = Suspend::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::Suspend(inner));
},
"set_title" => {
	let inner = SetTitle::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::SetTitle(inner));
},
"set_icon" => {
	let inner = SetIcon::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::SetIcon(inner));
},
"screenshot" => {
	let inner = Screenshot::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::Screenshot(inner));
},
"option_set" => {
	let inner = OptionSet::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::OptionSet(inner));
},
"chdir" => {
	let inner = Chdir::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::Chdir(inner));
},
"update_fg" => {
	let inner = UpdateFg::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::UpdateFg(inner));
},
"update_bg" => {
	let inner = UpdateBg::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::UpdateBg(inner));
},
"update_sp" => {
	let inner = UpdateSp::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::UpdateSp(inner));
},
"resize" => {
	let inner = Resize::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::Resize(inner));
},
"clear" => {
	let inner = Clear::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::Clear(inner));
},
"eol_clear" => {
	let inner = EolClear::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::EolClear(inner));
},
"cursor_goto" => {
	let inner = CursorGoto::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::CursorGoto(inner));
},
"highlight_set" => {
	let inner = HighlightSet::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::HighlightSet(inner));
},
"put" => {
	let inner = Put::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::Put(inner));
},
"set_scroll_region" => {
	let inner = SetScrollRegion::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::SetScrollRegion(inner));
},
"scroll" => {
	let inner = Scroll::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::Scroll(inner));
},
"default_colors_set" => {
	let inner = DefaultColorsSet::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::DefaultColorsSet(inner));
},
"hl_attr_define" => {
	let inner = HlAttrDefine::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::HlAttrDefine(inner));
},
"hl_group_set" => {
	let inner = HlGroupSet::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::HlGroupSet(inner));
},
"grid_resize" => {
	let inner = GridResize::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::GridResize(inner));
},
"grid_clear" => {
	let inner = GridClear::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::GridClear(inner));
},
"grid_cursor_goto" => {
	let inner = GridCursorGoto::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::GridCursorGoto(inner));
},
"grid_line" => {
	let inner = GridLine::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::GridLine(inner));
},
"grid_scroll" => {
	let inner = GridScroll::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::GridScroll(inner));
},
"grid_destroy" => {
	let inner = GridDestroy::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::GridDestroy(inner));
},
"win_pos" => {
	let inner = WinPos::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WinPos(inner));
},
"win_float_pos" => {
	let inner = WinFloatPos::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WinFloatPos(inner));
},
"win_external_pos" => {
	let inner = WinExternalPos::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WinExternalPos(inner));
},
"win_hide" => {
	let inner = WinHide::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WinHide(inner));
},
"win_close" => {
	let inner = WinClose::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WinClose(inner));
},
"msg_set_pos" => {
	let inner = MsgSetPos::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::MsgSetPos(inner));
},
"win_viewport" => {
	let inner = WinViewport::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WinViewport(inner));
},
"win_viewport_margins" => {
	let inner = WinViewportMargins::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WinViewportMargins(inner));
},
"win_extmark" => {
	let inner = WinExtmark::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WinExtmark(inner));
},
"popupmenu_show" => {
	let inner = PopupmenuShow::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::PopupmenuShow(inner));
},
"popupmenu_hide" => {
	let inner = PopupmenuHide::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::PopupmenuHide(inner));
},
"popupmenu_select" => {
	let inner = PopupmenuSelect::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::PopupmenuSelect(inner));
},
"tabline_update" => {
	let inner = TablineUpdate::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::TablineUpdate(inner));
},
"cmdline_show" => {
	let inner = CmdlineShow::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::CmdlineShow(inner));
},
"cmdline_pos" => {
	let inner = CmdlinePos::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::CmdlinePos(inner));
},
"cmdline_special_char" => {
	let inner = CmdlineSpecialChar::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::CmdlineSpecialChar(inner));
},
"cmdline_hide" => {
	let inner = CmdlineHide::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::CmdlineHide(inner));
},
"cmdline_block_show" => {
	let inner = CmdlineBlockShow::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::CmdlineBlockShow(inner));
},
"cmdline_block_append" => {
	let inner = CmdlineBlockAppend::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::CmdlineBlockAppend(inner));
},
"cmdline_block_hide" => {
	let inner = CmdlineBlockHide::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::CmdlineBlockHide(inner));
},
"wildmenu_show" => {
	let inner = WildmenuShow::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WildmenuShow(inner));
},
"wildmenu_select" => {
	let inner = WildmenuSelect::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WildmenuSelect(inner));
},
"wildmenu_hide" => {
	let inner = WildmenuHide::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WildmenuHide(inner));
},
"msg_show" => {
	let inner = MsgShow::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::MsgShow(inner));
},
"msg_clear" => {
	let inner = MsgClear::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::MsgClear(inner));
},
"msg_showcmd" => {
	let inner = MsgShowcmd::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::MsgShowcmd(inner));
},
"msg_showmode" => {
	let inner = MsgShowmode::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::MsgShowmode(inner));
},
"msg_ruler" => {
	let inner = MsgRuler::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::MsgRuler(inner));
},
"msg_history_show" => {
	let inner = MsgHistoryShow::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::MsgHistoryShow(inner));
},
"msg_history_clear" => {
	let inner = MsgHistoryClear::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::MsgHistoryClear(inner));
},
"error_exit" => {
	let inner = ErrorExit::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::ErrorExit(inner));
},

        o => {
            let inner = Value::deserialize(ContSeq::new(seq))?;
            return Ok(UiEvent::Unknown(o.to_string() ,inner));
        }
    
}
} } } }
#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UiOptions {
	Rgb,
	ExtCmdline,
	ExtPopupmenu,
	ExtTabline,
	ExtWildmenu,
	ExtMessages,
	ExtLinegrid,
	ExtMultigrid,
	ExtHlstate,
	ExtTermcolors,
}
