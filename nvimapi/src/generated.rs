use crate::Pairs;
use crate::TryFromValue;
use crate::contseq::ContSeq;
use crate::error;
use crate::nvimapi::{BUFFER_ID, TABPAGE_ID, WINDOW_ID};
use log::debug;
use rmpv::Value;
use serde::Deserializer;
use serde::{Deserialize, Serialize};
type Boolean = bool;
type Integer = i64;
type Float = f64;
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Buffer(pub Value);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Window(pub Value);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Tabpage(pub Value);
type Array = Vec<Value>;
type Dict = Pairs<Value, Value>;
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
        let Value::Ext(id, _) = &value else {
            return error::with_msg("expected msgpack ext.");
        };
        if *id != BUFFER_ID {
            return error::with_msg("expected id 0 for buffer");
        }
        return Ok(Self(value));
    }
}
impl TryFromValue for Window {
    fn try_from_value(value: Value) -> error::Result<Self> {
        let Value::Ext(id, _) = &value else {
            return error::with_msg("expected msgpack ext.");
        };
        if *id != WINDOW_ID {
            return error::with_msg("expected id 1 for window");
        }
        return Ok(Self(value));
    }
}
impl TryFromValue for Tabpage {
    fn try_from_value(value: Value) -> error::Result<Self> {
        let Value::Ext(id, _) = &value else {
            return error::with_msg("expected msgpack ext.");
        };
        if *id != TABPAGE_ID {
            return error::with_msg("expected id 2 for tabpage");
        }
        return Ok(Self(value));
    }
}

pub trait Nvimapi {
    fn noret(&self) -> &impl NvimapiNr;
    fn send_response_wv(&self, msgid: i32, error: Value, result: Value) -> error::Result<()>;
    fn send_response(
        &self,
        msgid: i32,
        error: impl serde::Serialize,
        result: impl serde::Serialize,
    ) -> error::Result<()>;
    async fn call_fn_wv<R>(
        &self,
        fn_name: String,
        args: impl crate::valueseq::ValueSeq,
    ) -> error::Result<R>
    where
        R: TryFromValue;
    async fn call_fn<D, S>(&self, fn_name: &str, args: S) -> error::Result<D>
    where
        D: Deserialize<'static>,
        S: crate::valueseq::SerialSeq;
    async fn get_autocmds<D: Deserialize<'static>>(
        &self,
        opts: impl Serialize,
    ) -> error::Result<D> {
        self.call_fn("nvim_get_autocmds".into(), (opts,)).await
    }
    async fn get_autocmds_wv(&self, opts: Dict) -> error::Result<Array> {
        self.call_fn_wv("nvim_get_autocmds".into(), (opts,)).await
    }
    async fn create_autocmd(
        &self,
        event: impl Serialize,
        opts: impl Serialize,
    ) -> error::Result<Integer> {
        self.call_fn("nvim_create_autocmd".into(), (event, opts))
            .await
    }
    async fn create_autocmd_wv(&self, event: Object, opts: Dict) -> error::Result<Integer> {
        self.call_fn_wv("nvim_create_autocmd".into(), (event, opts))
            .await
    }
    async fn del_autocmd(&self, id: Integer) -> error::Result<()> {
        self.call_fn("nvim_del_autocmd".into(), (id,)).await
    }
    async fn clear_autocmds(&self, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_clear_autocmds".into(), (opts,)).await
    }
    async fn clear_autocmds_wv(&self, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_clear_autocmds".into(), (opts,)).await
    }
    async fn create_augroup(&self, name: &str, opts: impl Serialize) -> error::Result<Integer> {
        self.call_fn("nvim_create_augroup".into(), (name, opts))
            .await
    }
    async fn create_augroup_wv(&self, name: String, opts: Dict) -> error::Result<Integer> {
        self.call_fn_wv("nvim_create_augroup".into(), (name, opts))
            .await
    }
    async fn del_augroup_by_id(&self, id: Integer) -> error::Result<()> {
        self.call_fn("nvim_del_augroup_by_id".into(), (id,)).await
    }
    async fn del_augroup_by_name(&self, name: &str) -> error::Result<()> {
        self.call_fn("nvim_del_augroup_by_name".into(), (name,))
            .await
    }
    async fn exec_autocmds(
        &self,
        event: impl Serialize,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_exec_autocmds".into(), (event, opts))
            .await
    }
    async fn exec_autocmds_wv(&self, event: Object, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_exec_autocmds".into(), (event, opts))
            .await
    }
    async fn buf_line_count(&self, buffer: Buffer) -> error::Result<Integer> {
        self.call_fn("nvim_buf_line_count".into(), (buffer,)).await
    }
    async fn buf_attach(
        &self,
        buffer: Buffer,
        send_buffer: Boolean,
        opts: impl Serialize,
    ) -> error::Result<Boolean> {
        self.call_fn("nvim_buf_attach".into(), (buffer, send_buffer, opts))
            .await
    }
    async fn buf_attach_wv(
        &self,
        buffer: Buffer,
        send_buffer: Boolean,
        opts: Dict,
    ) -> error::Result<Boolean> {
        self.call_fn_wv("nvim_buf_attach".into(), (buffer, send_buffer, opts))
            .await
    }
    async fn buf_detach(&self, buffer: Buffer) -> error::Result<Boolean> {
        self.call_fn("nvim_buf_detach".into(), (buffer,)).await
    }
    async fn buf_get_lines(
        &self,
        buffer: Buffer,
        start: Integer,
        end: Integer,
        strict_indexing: Boolean,
    ) -> error::Result<Vec<String>> {
        self.call_fn(
            "nvim_buf_get_lines".into(),
            (buffer, start, end, strict_indexing),
        )
        .await
    }
    async fn buf_set_lines(
        &self,
        buffer: Buffer,
        start: Integer,
        end: Integer,
        strict_indexing: Boolean,
        replacement: &[&str],
    ) -> error::Result<()> {
        self.call_fn(
            "nvim_buf_set_lines".into(),
            (buffer, start, end, strict_indexing, replacement),
        )
        .await
    }
    async fn buf_set_text(
        &self,
        buffer: Buffer,
        start_row: Integer,
        start_col: Integer,
        end_row: Integer,
        end_col: Integer,
        replacement: &[&str],
    ) -> error::Result<()> {
        self.call_fn(
            "nvim_buf_set_text".into(),
            (buffer, start_row, start_col, end_row, end_col, replacement),
        )
        .await
    }
    async fn buf_get_text(
        &self,
        buffer: Buffer,
        start_row: Integer,
        start_col: Integer,
        end_row: Integer,
        end_col: Integer,
        opts: impl Serialize,
    ) -> error::Result<Vec<String>> {
        self.call_fn(
            "nvim_buf_get_text".into(),
            (buffer, start_row, start_col, end_row, end_col, opts),
        )
        .await
    }
    async fn buf_get_text_wv(
        &self,
        buffer: Buffer,
        start_row: Integer,
        start_col: Integer,
        end_row: Integer,
        end_col: Integer,
        opts: Dict,
    ) -> error::Result<Vec<String>> {
        self.call_fn_wv(
            "nvim_buf_get_text".into(),
            (buffer, start_row, start_col, end_row, end_col, opts),
        )
        .await
    }
    async fn buf_get_offset(&self, buffer: Buffer, index: Integer) -> error::Result<Integer> {
        self.call_fn("nvim_buf_get_offset".into(), (buffer, index))
            .await
    }
    async fn buf_get_var<D: Deserialize<'static>>(
        &self,
        buffer: Buffer,
        name: &str,
    ) -> error::Result<D> {
        self.call_fn("nvim_buf_get_var".into(), (buffer, name))
            .await
    }
    async fn buf_get_var_wv(&self, buffer: Buffer, name: String) -> error::Result<Object> {
        self.call_fn_wv("nvim_buf_get_var".into(), (buffer, name))
            .await
    }
    async fn buf_get_changedtick(&self, buffer: Buffer) -> error::Result<Integer> {
        self.call_fn("nvim_buf_get_changedtick".into(), (buffer,))
            .await
    }
    async fn buf_get_keymap<D: Deserialize<'static>>(
        &self,
        buffer: Buffer,
        mode: &str,
    ) -> error::Result<D> {
        self.call_fn("nvim_buf_get_keymap".into(), (buffer, mode))
            .await
    }
    async fn buf_get_keymap_wv(&self, buffer: Buffer, mode: String) -> error::Result<Vec<Dict>> {
        self.call_fn_wv("nvim_buf_get_keymap".into(), (buffer, mode))
            .await
    }
    async fn buf_set_keymap(
        &self,
        buffer: Buffer,
        mode: &str,
        lhs: &str,
        rhs: &str,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_buf_set_keymap".into(), (buffer, mode, lhs, rhs, opts))
            .await
    }
    async fn buf_set_keymap_wv(
        &self,
        buffer: Buffer,
        mode: String,
        lhs: String,
        rhs: String,
        opts: Dict,
    ) -> error::Result<()> {
        self.call_fn_wv("nvim_buf_set_keymap".into(), (buffer, mode, lhs, rhs, opts))
            .await
    }
    async fn buf_del_keymap(&self, buffer: Buffer, mode: &str, lhs: &str) -> error::Result<()> {
        self.call_fn("nvim_buf_del_keymap".into(), (buffer, mode, lhs))
            .await
    }
    async fn buf_set_var(
        &self,
        buffer: Buffer,
        name: &str,
        value: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_buf_set_var".into(), (buffer, name, value))
            .await
    }
    async fn buf_set_var_wv(
        &self,
        buffer: Buffer,
        name: String,
        value: Object,
    ) -> error::Result<()> {
        self.call_fn_wv("nvim_buf_set_var".into(), (buffer, name, value))
            .await
    }
    async fn buf_del_var(&self, buffer: Buffer, name: &str) -> error::Result<()> {
        self.call_fn("nvim_buf_del_var".into(), (buffer, name))
            .await
    }
    async fn buf_get_name(&self, buffer: Buffer) -> error::Result<String> {
        self.call_fn("nvim_buf_get_name".into(), (buffer,)).await
    }
    async fn buf_set_name(&self, buffer: Buffer, name: &str) -> error::Result<()> {
        self.call_fn("nvim_buf_set_name".into(), (buffer, name))
            .await
    }
    async fn buf_is_loaded(&self, buffer: Buffer) -> error::Result<Boolean> {
        self.call_fn("nvim_buf_is_loaded".into(), (buffer,)).await
    }
    async fn buf_delete(&self, buffer: Buffer, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_buf_delete".into(), (buffer, opts)).await
    }
    async fn buf_delete_wv(&self, buffer: Buffer, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_buf_delete".into(), (buffer, opts))
            .await
    }
    async fn buf_is_valid(&self, buffer: Buffer) -> error::Result<Boolean> {
        self.call_fn("nvim_buf_is_valid".into(), (buffer,)).await
    }
    async fn buf_del_mark(&self, buffer: Buffer, name: &str) -> error::Result<Boolean> {
        self.call_fn("nvim_buf_del_mark".into(), (buffer, name))
            .await
    }
    async fn buf_set_mark(
        &self,
        buffer: Buffer,
        name: &str,
        line: Integer,
        col: Integer,
        opts: impl Serialize,
    ) -> error::Result<Boolean> {
        self.call_fn("nvim_buf_set_mark".into(), (buffer, name, line, col, opts))
            .await
    }
    async fn buf_set_mark_wv(
        &self,
        buffer: Buffer,
        name: String,
        line: Integer,
        col: Integer,
        opts: Dict,
    ) -> error::Result<Boolean> {
        self.call_fn_wv("nvim_buf_set_mark".into(), (buffer, name, line, col, opts))
            .await
    }
    async fn buf_get_mark(&self, buffer: Buffer, name: &str) -> error::Result<Vec<Integer>> {
        self.call_fn("nvim_buf_get_mark".into(), (buffer, name))
            .await
    }
    async fn parse_cmd<D: Deserialize<'static>>(
        &self,
        str: &str,
        opts: impl Serialize,
    ) -> error::Result<D> {
        self.call_fn("nvim_parse_cmd".into(), (str, opts)).await
    }
    async fn parse_cmd_wv(&self, str: String, opts: Dict) -> error::Result<Dict> {
        self.call_fn_wv("nvim_parse_cmd".into(), (str, opts)).await
    }
    async fn cmd(&self, cmd: impl Serialize, opts: impl Serialize) -> error::Result<String> {
        self.call_fn("nvim_cmd".into(), (cmd, opts)).await
    }
    async fn cmd_wv(&self, cmd: Dict, opts: Dict) -> error::Result<String> {
        self.call_fn_wv("nvim_cmd".into(), (cmd, opts)).await
    }
    async fn create_user_command(
        &self,
        name: &str,
        command: impl Serialize,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_create_user_command".into(), (name, command, opts))
            .await
    }
    async fn create_user_command_wv(
        &self,
        name: String,
        command: Object,
        opts: Dict,
    ) -> error::Result<()> {
        self.call_fn_wv("nvim_create_user_command".into(), (name, command, opts))
            .await
    }
    async fn del_user_command(&self, name: &str) -> error::Result<()> {
        self.call_fn("nvim_del_user_command".into(), (name,)).await
    }
    async fn buf_create_user_command(
        &self,
        buffer: Buffer,
        name: &str,
        command: impl Serialize,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn(
            "nvim_buf_create_user_command".into(),
            (buffer, name, command, opts),
        )
        .await
    }
    async fn buf_create_user_command_wv(
        &self,
        buffer: Buffer,
        name: String,
        command: Object,
        opts: Dict,
    ) -> error::Result<()> {
        self.call_fn_wv(
            "nvim_buf_create_user_command".into(),
            (buffer, name, command, opts),
        )
        .await
    }
    async fn buf_del_user_command(&self, buffer: Buffer, name: &str) -> error::Result<()> {
        self.call_fn("nvim_buf_del_user_command".into(), (buffer, name))
            .await
    }
    async fn get_commands<D: Deserialize<'static>>(
        &self,
        opts: impl Serialize,
    ) -> error::Result<D> {
        self.call_fn("nvim_get_commands".into(), (opts,)).await
    }
    async fn get_commands_wv(&self, opts: Dict) -> error::Result<Dict> {
        self.call_fn_wv("nvim_get_commands".into(), (opts,)).await
    }
    async fn buf_get_commands<D: Deserialize<'static>>(
        &self,
        buffer: Buffer,
        opts: impl Serialize,
    ) -> error::Result<D> {
        self.call_fn("nvim_buf_get_commands".into(), (buffer, opts))
            .await
    }
    async fn buf_get_commands_wv(&self, buffer: Buffer, opts: Dict) -> error::Result<Dict> {
        self.call_fn_wv("nvim_buf_get_commands".into(), (buffer, opts))
            .await
    }
    async fn create_namespace(&self, name: &str) -> error::Result<Integer> {
        self.call_fn("nvim_create_namespace".into(), (name,)).await
    }
    async fn get_namespaces<D: Deserialize<'static>>(&self) -> error::Result<D> {
        self.call_fn("nvim_get_namespaces".into(), [(); 0]).await
    }
    async fn get_namespaces_wv(&self) -> error::Result<Dict> {
        self.call_fn_wv("nvim_get_namespaces".into(), [(); 0]).await
    }
    async fn buf_get_extmark_by_id(
        &self,
        buffer: Buffer,
        ns_id: Integer,
        id: Integer,
        opts: impl Serialize,
    ) -> error::Result<Vec<Integer>> {
        self.call_fn(
            "nvim_buf_get_extmark_by_id".into(),
            (buffer, ns_id, id, opts),
        )
        .await
    }
    async fn buf_get_extmark_by_id_wv(
        &self,
        buffer: Buffer,
        ns_id: Integer,
        id: Integer,
        opts: Dict,
    ) -> error::Result<Vec<Integer>> {
        self.call_fn_wv(
            "nvim_buf_get_extmark_by_id".into(),
            (buffer, ns_id, id, opts),
        )
        .await
    }
    async fn buf_get_extmarks<D: Deserialize<'static>>(
        &self,
        buffer: Buffer,
        ns_id: Integer,
        start: impl Serialize,
        end: impl Serialize,
        opts: impl Serialize,
    ) -> error::Result<D> {
        self.call_fn(
            "nvim_buf_get_extmarks".into(),
            (buffer, ns_id, start, end, opts),
        )
        .await
    }
    async fn buf_get_extmarks_wv(
        &self,
        buffer: Buffer,
        ns_id: Integer,
        start: Object,
        end: Object,
        opts: Dict,
    ) -> error::Result<Array> {
        self.call_fn_wv(
            "nvim_buf_get_extmarks".into(),
            (buffer, ns_id, start, end, opts),
        )
        .await
    }
    async fn buf_set_extmark(
        &self,
        buffer: Buffer,
        ns_id: Integer,
        line: Integer,
        col: Integer,
        opts: impl Serialize,
    ) -> error::Result<Integer> {
        self.call_fn(
            "nvim_buf_set_extmark".into(),
            (buffer, ns_id, line, col, opts),
        )
        .await
    }
    async fn buf_set_extmark_wv(
        &self,
        buffer: Buffer,
        ns_id: Integer,
        line: Integer,
        col: Integer,
        opts: Dict,
    ) -> error::Result<Integer> {
        self.call_fn_wv(
            "nvim_buf_set_extmark".into(),
            (buffer, ns_id, line, col, opts),
        )
        .await
    }
    async fn buf_del_extmark(
        &self,
        buffer: Buffer,
        ns_id: Integer,
        id: Integer,
    ) -> error::Result<Boolean> {
        self.call_fn("nvim_buf_del_extmark".into(), (buffer, ns_id, id))
            .await
    }
    async fn buf_clear_namespace(
        &self,
        buffer: Buffer,
        ns_id: Integer,
        line_start: Integer,
        line_end: Integer,
    ) -> error::Result<()> {
        self.call_fn(
            "nvim_buf_clear_namespace".into(),
            (buffer, ns_id, line_start, line_end),
        )
        .await
    }
    async fn set_decoration_provider(
        &self,
        ns_id: Integer,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_set_decoration_provider".into(), (ns_id, opts))
            .await
    }
    async fn set_decoration_provider_wv(&self, ns_id: Integer, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_set_decoration_provider".into(), (ns_id, opts))
            .await
    }
    async fn get_option_value<D: Deserialize<'static>>(
        &self,
        name: &str,
        opts: impl Serialize,
    ) -> error::Result<D> {
        self.call_fn("nvim_get_option_value".into(), (name, opts))
            .await
    }
    async fn get_option_value_wv(&self, name: String, opts: Dict) -> error::Result<Object> {
        self.call_fn_wv("nvim_get_option_value".into(), (name, opts))
            .await
    }
    async fn set_option_value(
        &self,
        name: &str,
        value: impl Serialize,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_set_option_value".into(), (name, value, opts))
            .await
    }
    async fn set_option_value_wv(
        &self,
        name: String,
        value: Object,
        opts: Dict,
    ) -> error::Result<()> {
        self.call_fn_wv("nvim_set_option_value".into(), (name, value, opts))
            .await
    }
    async fn get_all_options_info<D: Deserialize<'static>>(&self) -> error::Result<D> {
        self.call_fn("nvim_get_all_options_info".into(), [(); 0])
            .await
    }
    async fn get_all_options_info_wv(&self) -> error::Result<Dict> {
        self.call_fn_wv("nvim_get_all_options_info".into(), [(); 0])
            .await
    }
    async fn get_option_info2<D: Deserialize<'static>>(
        &self,
        name: &str,
        opts: impl Serialize,
    ) -> error::Result<D> {
        self.call_fn("nvim_get_option_info2".into(), (name, opts))
            .await
    }
    async fn get_option_info2_wv(&self, name: String, opts: Dict) -> error::Result<Dict> {
        self.call_fn_wv("nvim_get_option_info2".into(), (name, opts))
            .await
    }
    async fn tabpage_list_wins(&self, tabpage: Tabpage) -> error::Result<Vec<Window>> {
        self.call_fn("nvim_tabpage_list_wins".into(), (tabpage,))
            .await
    }
    async fn tabpage_get_var<D: Deserialize<'static>>(
        &self,
        tabpage: Tabpage,
        name: &str,
    ) -> error::Result<D> {
        self.call_fn("nvim_tabpage_get_var".into(), (tabpage, name))
            .await
    }
    async fn tabpage_get_var_wv(&self, tabpage: Tabpage, name: String) -> error::Result<Object> {
        self.call_fn_wv("nvim_tabpage_get_var".into(), (tabpage, name))
            .await
    }
    async fn tabpage_set_var(
        &self,
        tabpage: Tabpage,
        name: &str,
        value: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_tabpage_set_var".into(), (tabpage, name, value))
            .await
    }
    async fn tabpage_set_var_wv(
        &self,
        tabpage: Tabpage,
        name: String,
        value: Object,
    ) -> error::Result<()> {
        self.call_fn_wv("nvim_tabpage_set_var".into(), (tabpage, name, value))
            .await
    }
    async fn tabpage_del_var(&self, tabpage: Tabpage, name: &str) -> error::Result<()> {
        self.call_fn("nvim_tabpage_del_var".into(), (tabpage, name))
            .await
    }
    async fn tabpage_get_win(&self, tabpage: Tabpage) -> error::Result<Window> {
        self.call_fn("nvim_tabpage_get_win".into(), (tabpage,))
            .await
    }
    async fn tabpage_set_win(&self, tabpage: Tabpage, win: Window) -> error::Result<()> {
        self.call_fn("nvim_tabpage_set_win".into(), (tabpage, win))
            .await
    }
    async fn tabpage_get_number(&self, tabpage: Tabpage) -> error::Result<Integer> {
        self.call_fn("nvim_tabpage_get_number".into(), (tabpage,))
            .await
    }
    async fn tabpage_is_valid(&self, tabpage: Tabpage) -> error::Result<Boolean> {
        self.call_fn("nvim_tabpage_is_valid".into(), (tabpage,))
            .await
    }
    async fn ui_attach(
        &self,
        width: Integer,
        height: Integer,
        options: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_ui_attach".into(), (width, height, options))
            .await
    }
    async fn ui_attach_wv(
        &self,
        width: Integer,
        height: Integer,
        options: Dict,
    ) -> error::Result<()> {
        self.call_fn_wv("nvim_ui_attach".into(), (width, height, options))
            .await
    }
    async fn ui_set_focus(&self, gained: Boolean) -> error::Result<()> {
        self.call_fn("nvim_ui_set_focus".into(), (gained,)).await
    }
    async fn ui_detach(&self) -> error::Result<()> {
        self.call_fn("nvim_ui_detach".into(), [(); 0]).await
    }
    async fn ui_try_resize(&self, width: Integer, height: Integer) -> error::Result<()> {
        self.call_fn("nvim_ui_try_resize".into(), (width, height))
            .await
    }
    async fn ui_set_option(&self, name: &str, value: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_ui_set_option".into(), (name, value))
            .await
    }
    async fn ui_set_option_wv(&self, name: String, value: Object) -> error::Result<()> {
        self.call_fn_wv("nvim_ui_set_option".into(), (name, value))
            .await
    }
    async fn ui_try_resize_grid(
        &self,
        grid: Integer,
        width: Integer,
        height: Integer,
    ) -> error::Result<()> {
        self.call_fn("nvim_ui_try_resize_grid".into(), (grid, width, height))
            .await
    }
    async fn ui_pum_set_height(&self, height: Integer) -> error::Result<()> {
        self.call_fn("nvim_ui_pum_set_height".into(), (height,))
            .await
    }
    async fn ui_pum_set_bounds(
        &self,
        width: Float,
        height: Float,
        row: Float,
        col: Float,
    ) -> error::Result<()> {
        self.call_fn("nvim_ui_pum_set_bounds".into(), (width, height, row, col))
            .await
    }
    async fn ui_term_event(&self, event: &str, value: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_ui_term_event".into(), (event, value))
            .await
    }
    async fn ui_term_event_wv(&self, event: String, value: Object) -> error::Result<()> {
        self.call_fn_wv("nvim_ui_term_event".into(), (event, value))
            .await
    }
    async fn get_hl_id_by_name(&self, name: &str) -> error::Result<Integer> {
        self.call_fn("nvim_get_hl_id_by_name".into(), (name,)).await
    }
    async fn get_hl<D: Deserialize<'static>>(
        &self,
        ns_id: Integer,
        opts: impl Serialize,
    ) -> error::Result<D> {
        self.call_fn("nvim_get_hl".into(), (ns_id, opts)).await
    }
    async fn get_hl_wv(&self, ns_id: Integer, opts: Dict) -> error::Result<Dict> {
        self.call_fn_wv("nvim_get_hl".into(), (ns_id, opts)).await
    }
    async fn set_hl(&self, ns_id: Integer, name: &str, val: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_set_hl".into(), (ns_id, name, val)).await
    }
    async fn set_hl_wv(&self, ns_id: Integer, name: String, val: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_set_hl".into(), (ns_id, name, val))
            .await
    }
    async fn get_hl_ns(&self, opts: impl Serialize) -> error::Result<Integer> {
        self.call_fn("nvim_get_hl_ns".into(), (opts,)).await
    }
    async fn get_hl_ns_wv(&self, opts: Dict) -> error::Result<Integer> {
        self.call_fn_wv("nvim_get_hl_ns".into(), (opts,)).await
    }
    async fn set_hl_ns(&self, ns_id: Integer) -> error::Result<()> {
        self.call_fn("nvim_set_hl_ns".into(), (ns_id,)).await
    }
    async fn set_hl_ns_fast(&self, ns_id: Integer) -> error::Result<()> {
        self.call_fn("nvim_set_hl_ns_fast".into(), (ns_id,)).await
    }
    async fn feedkeys(&self, keys: &str, mode: &str, escape_ks: Boolean) -> error::Result<()> {
        self.call_fn("nvim_feedkeys".into(), (keys, mode, escape_ks))
            .await
    }
    async fn input(&self, keys: &str) -> error::Result<Integer> {
        self.call_fn("nvim_input".into(), (keys,)).await
    }
    async fn input_mouse(
        &self,
        button: &str,
        action: &str,
        modifier: &str,
        grid: Integer,
        row: Integer,
        col: Integer,
    ) -> error::Result<()> {
        self.call_fn(
            "nvim_input_mouse".into(),
            (button, action, modifier, grid, row, col),
        )
        .await
    }
    async fn replace_termcodes(
        &self,
        str: &str,
        from_part: Boolean,
        do_lt: Boolean,
        special: Boolean,
    ) -> error::Result<String> {
        self.call_fn(
            "nvim_replace_termcodes".into(),
            (str, from_part, do_lt, special),
        )
        .await
    }
    async fn exec_lua<D: Deserialize<'static>>(
        &self,
        code: &str,
        args: impl Serialize,
    ) -> error::Result<D> {
        self.call_fn("nvim_exec_lua".into(), (code, args)).await
    }
    async fn exec_lua_wv(&self, code: String, args: Array) -> error::Result<Object> {
        self.call_fn_wv("nvim_exec_lua".into(), (code, args)).await
    }
    async fn strwidth(&self, text: &str) -> error::Result<Integer> {
        self.call_fn("nvim_strwidth".into(), (text,)).await
    }
    async fn list_runtime_paths(&self) -> error::Result<Vec<String>> {
        self.call_fn("nvim_list_runtime_paths".into(), [(); 0])
            .await
    }
    async fn get_runtime_file(&self, name: &str, all: Boolean) -> error::Result<Vec<String>> {
        self.call_fn("nvim_get_runtime_file".into(), (name, all))
            .await
    }
    async fn set_current_dir(&self, dir: &str) -> error::Result<()> {
        self.call_fn("nvim_set_current_dir".into(), (dir,)).await
    }
    async fn get_current_line(&self) -> error::Result<String> {
        self.call_fn("nvim_get_current_line".into(), [(); 0]).await
    }
    async fn set_current_line(&self, line: &str) -> error::Result<()> {
        self.call_fn("nvim_set_current_line".into(), (line,)).await
    }
    async fn del_current_line(&self) -> error::Result<()> {
        self.call_fn("nvim_del_current_line".into(), [(); 0]).await
    }
    async fn get_var<D: Deserialize<'static>>(&self, name: &str) -> error::Result<D> {
        self.call_fn("nvim_get_var".into(), (name,)).await
    }
    async fn get_var_wv(&self, name: String) -> error::Result<Object> {
        self.call_fn_wv("nvim_get_var".into(), (name,)).await
    }
    async fn set_var(&self, name: &str, value: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_set_var".into(), (name, value)).await
    }
    async fn set_var_wv(&self, name: String, value: Object) -> error::Result<()> {
        self.call_fn_wv("nvim_set_var".into(), (name, value)).await
    }
    async fn del_var(&self, name: &str) -> error::Result<()> {
        self.call_fn("nvim_del_var".into(), (name,)).await
    }
    async fn get_vvar<D: Deserialize<'static>>(&self, name: &str) -> error::Result<D> {
        self.call_fn("nvim_get_vvar".into(), (name,)).await
    }
    async fn get_vvar_wv(&self, name: String) -> error::Result<Object> {
        self.call_fn_wv("nvim_get_vvar".into(), (name,)).await
    }
    async fn set_vvar(&self, name: &str, value: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_set_vvar".into(), (name, value)).await
    }
    async fn set_vvar_wv(&self, name: String, value: Object) -> error::Result<()> {
        self.call_fn_wv("nvim_set_vvar".into(), (name, value)).await
    }
    async fn echo(
        &self,
        chunks: impl Serialize,
        history: Boolean,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_echo".into(), (chunks, history, opts))
            .await
    }
    async fn echo_wv(&self, chunks: Array, history: Boolean, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_echo".into(), (chunks, history, opts))
            .await
    }
    async fn list_bufs(&self) -> error::Result<Vec<Buffer>> {
        self.call_fn("nvim_list_bufs".into(), [(); 0]).await
    }
    async fn get_current_buf(&self) -> error::Result<Buffer> {
        self.call_fn("nvim_get_current_buf".into(), [(); 0]).await
    }
    async fn set_current_buf(&self, buffer: Buffer) -> error::Result<()> {
        self.call_fn("nvim_set_current_buf".into(), (buffer,)).await
    }
    async fn list_wins(&self) -> error::Result<Vec<Window>> {
        self.call_fn("nvim_list_wins".into(), [(); 0]).await
    }
    async fn get_current_win(&self) -> error::Result<Window> {
        self.call_fn("nvim_get_current_win".into(), [(); 0]).await
    }
    async fn set_current_win(&self, window: Window) -> error::Result<()> {
        self.call_fn("nvim_set_current_win".into(), (window,)).await
    }
    async fn create_buf(&self, listed: Boolean, scratch: Boolean) -> error::Result<Buffer> {
        self.call_fn("nvim_create_buf".into(), (listed, scratch))
            .await
    }
    async fn open_term(&self, buffer: Buffer, opts: impl Serialize) -> error::Result<Integer> {
        self.call_fn("nvim_open_term".into(), (buffer, opts)).await
    }
    async fn open_term_wv(&self, buffer: Buffer, opts: Dict) -> error::Result<Integer> {
        self.call_fn_wv("nvim_open_term".into(), (buffer, opts))
            .await
    }
    async fn chan_send(&self, chan: Integer, data: &str) -> error::Result<()> {
        self.call_fn("nvim_chan_send".into(), (chan, data)).await
    }
    async fn list_tabpages(&self) -> error::Result<Vec<Tabpage>> {
        self.call_fn("nvim_list_tabpages".into(), [(); 0]).await
    }
    async fn get_current_tabpage(&self) -> error::Result<Tabpage> {
        self.call_fn("nvim_get_current_tabpage".into(), [(); 0])
            .await
    }
    async fn set_current_tabpage(&self, tabpage: Tabpage) -> error::Result<()> {
        self.call_fn("nvim_set_current_tabpage".into(), (tabpage,))
            .await
    }
    async fn paste(&self, data: &str, crlf: Boolean, phase: Integer) -> error::Result<Boolean> {
        self.call_fn("nvim_paste".into(), (data, crlf, phase)).await
    }
    async fn put(
        &self,
        lines: &[&str],
        type_: &str,
        after: Boolean,
        follow: Boolean,
    ) -> error::Result<()> {
        self.call_fn("nvim_put".into(), (lines, type_, after, follow))
            .await
    }
    async fn get_color_by_name(&self, name: &str) -> error::Result<Integer> {
        self.call_fn("nvim_get_color_by_name".into(), (name,)).await
    }
    async fn get_color_map<D: Deserialize<'static>>(&self) -> error::Result<D> {
        self.call_fn("nvim_get_color_map".into(), [(); 0]).await
    }
    async fn get_color_map_wv(&self) -> error::Result<Dict> {
        self.call_fn_wv("nvim_get_color_map".into(), [(); 0]).await
    }
    async fn get_context<D: Deserialize<'static>>(&self, opts: impl Serialize) -> error::Result<D> {
        self.call_fn("nvim_get_context".into(), (opts,)).await
    }
    async fn get_context_wv(&self, opts: Dict) -> error::Result<Dict> {
        self.call_fn_wv("nvim_get_context".into(), (opts,)).await
    }
    async fn load_context<D: Deserialize<'static>>(
        &self,
        dict: impl Serialize,
    ) -> error::Result<D> {
        self.call_fn("nvim_load_context".into(), (dict,)).await
    }
    async fn load_context_wv(&self, dict: Dict) -> error::Result<Object> {
        self.call_fn_wv("nvim_load_context".into(), (dict,)).await
    }
    async fn get_mode<D: Deserialize<'static>>(&self) -> error::Result<D> {
        self.call_fn("nvim_get_mode".into(), [(); 0]).await
    }
    async fn get_mode_wv(&self) -> error::Result<Dict> {
        self.call_fn_wv("nvim_get_mode".into(), [(); 0]).await
    }
    async fn get_keymap<D: Deserialize<'static>>(&self, mode: &str) -> error::Result<D> {
        self.call_fn("nvim_get_keymap".into(), (mode,)).await
    }
    async fn get_keymap_wv(&self, mode: String) -> error::Result<Vec<Dict>> {
        self.call_fn_wv("nvim_get_keymap".into(), (mode,)).await
    }
    async fn set_keymap(
        &self,
        mode: &str,
        lhs: &str,
        rhs: &str,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_set_keymap".into(), (mode, lhs, rhs, opts))
            .await
    }
    async fn set_keymap_wv(
        &self,
        mode: String,
        lhs: String,
        rhs: String,
        opts: Dict,
    ) -> error::Result<()> {
        self.call_fn_wv("nvim_set_keymap".into(), (mode, lhs, rhs, opts))
            .await
    }
    async fn del_keymap(&self, mode: &str, lhs: &str) -> error::Result<()> {
        self.call_fn("nvim_del_keymap".into(), (mode, lhs)).await
    }
    async fn get_api_info<D: Deserialize<'static>>(&self) -> error::Result<D> {
        self.call_fn("nvim_get_api_info".into(), [(); 0]).await
    }
    async fn get_api_info_wv(&self) -> error::Result<Array> {
        self.call_fn_wv("nvim_get_api_info".into(), [(); 0]).await
    }
    async fn set_client_info(
        &self,
        name: &str,
        version: impl Serialize,
        type_: &str,
        methods: impl Serialize,
        attributes: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn(
            "nvim_set_client_info".into(),
            (name, version, type_, methods, attributes),
        )
        .await
    }
    async fn set_client_info_wv(
        &self,
        name: String,
        version: Dict,
        type_: String,
        methods: Dict,
        attributes: Dict,
    ) -> error::Result<()> {
        self.call_fn_wv(
            "nvim_set_client_info".into(),
            (name, version, type_, methods, attributes),
        )
        .await
    }
    async fn get_chan_info<D: Deserialize<'static>>(&self, chan: Integer) -> error::Result<D> {
        self.call_fn("nvim_get_chan_info".into(), (chan,)).await
    }
    async fn get_chan_info_wv(&self, chan: Integer) -> error::Result<Dict> {
        self.call_fn_wv("nvim_get_chan_info".into(), (chan,)).await
    }
    async fn list_chans<D: Deserialize<'static>>(&self) -> error::Result<D> {
        self.call_fn("nvim_list_chans".into(), [(); 0]).await
    }
    async fn list_chans_wv(&self) -> error::Result<Array> {
        self.call_fn_wv("nvim_list_chans".into(), [(); 0]).await
    }
    async fn list_uis<D: Deserialize<'static>>(&self) -> error::Result<D> {
        self.call_fn("nvim_list_uis".into(), [(); 0]).await
    }
    async fn list_uis_wv(&self) -> error::Result<Array> {
        self.call_fn_wv("nvim_list_uis".into(), [(); 0]).await
    }
    async fn get_proc_children<D: Deserialize<'static>>(&self, pid: Integer) -> error::Result<D> {
        self.call_fn("nvim_get_proc_children".into(), (pid,)).await
    }
    async fn get_proc_children_wv(&self, pid: Integer) -> error::Result<Array> {
        self.call_fn_wv("nvim_get_proc_children".into(), (pid,))
            .await
    }
    async fn get_proc<D: Deserialize<'static>>(&self, pid: Integer) -> error::Result<D> {
        self.call_fn("nvim_get_proc".into(), (pid,)).await
    }
    async fn get_proc_wv(&self, pid: Integer) -> error::Result<Object> {
        self.call_fn_wv("nvim_get_proc".into(), (pid,)).await
    }
    async fn select_popupmenu_item(
        &self,
        item: Integer,
        insert: Boolean,
        finish: Boolean,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn(
            "nvim_select_popupmenu_item".into(),
            (item, insert, finish, opts),
        )
        .await
    }
    async fn select_popupmenu_item_wv(
        &self,
        item: Integer,
        insert: Boolean,
        finish: Boolean,
        opts: Dict,
    ) -> error::Result<()> {
        self.call_fn_wv(
            "nvim_select_popupmenu_item".into(),
            (item, insert, finish, opts),
        )
        .await
    }
    async fn del_mark(&self, name: &str) -> error::Result<Boolean> {
        self.call_fn("nvim_del_mark".into(), (name,)).await
    }
    async fn get_mark<D: Deserialize<'static>>(
        &self,
        name: &str,
        opts: impl Serialize,
    ) -> error::Result<D> {
        self.call_fn("nvim_get_mark".into(), (name, opts)).await
    }
    async fn get_mark_wv(&self, name: String, opts: Dict) -> error::Result<Array> {
        self.call_fn_wv("nvim_get_mark".into(), (name, opts)).await
    }
    async fn eval_statusline<D: Deserialize<'static>>(
        &self,
        str: &str,
        opts: impl Serialize,
    ) -> error::Result<D> {
        self.call_fn("nvim_eval_statusline".into(), (str, opts))
            .await
    }
    async fn eval_statusline_wv(&self, str: String, opts: Dict) -> error::Result<Dict> {
        self.call_fn_wv("nvim_eval_statusline".into(), (str, opts))
            .await
    }
    async fn exec2<D: Deserialize<'static>>(
        &self,
        src: &str,
        opts: impl Serialize,
    ) -> error::Result<D> {
        self.call_fn("nvim_exec2".into(), (src, opts)).await
    }
    async fn exec2_wv(&self, src: String, opts: Dict) -> error::Result<Dict> {
        self.call_fn_wv("nvim_exec2".into(), (src, opts)).await
    }
    async fn command(&self, command: &str) -> error::Result<()> {
        self.call_fn("nvim_command".into(), (command,)).await
    }
    async fn eval<D: Deserialize<'static>>(&self, expr: &str) -> error::Result<D> {
        self.call_fn("nvim_eval".into(), (expr,)).await
    }
    async fn eval_wv(&self, expr: String) -> error::Result<Object> {
        self.call_fn_wv("nvim_eval".into(), (expr,)).await
    }
    async fn call_function<D: Deserialize<'static>>(
        &self,
        fn_: &str,
        args: impl Serialize,
    ) -> error::Result<D> {
        self.call_fn("nvim_call_function".into(), (fn_, args)).await
    }
    async fn call_function_wv(&self, fn_: String, args: Array) -> error::Result<Object> {
        self.call_fn_wv("nvim_call_function".into(), (fn_, args))
            .await
    }
    async fn call_dict_function<D: Deserialize<'static>>(
        &self,
        dict: impl Serialize,
        fn_: &str,
        args: impl Serialize,
    ) -> error::Result<D> {
        self.call_fn("nvim_call_dict_function".into(), (dict, fn_, args))
            .await
    }
    async fn call_dict_function_wv(
        &self,
        dict: Object,
        fn_: String,
        args: Array,
    ) -> error::Result<Object> {
        self.call_fn_wv("nvim_call_dict_function".into(), (dict, fn_, args))
            .await
    }
    async fn parse_expression<D: Deserialize<'static>>(
        &self,
        expr: &str,
        flags: &str,
        highlight: Boolean,
    ) -> error::Result<D> {
        self.call_fn("nvim_parse_expression".into(), (expr, flags, highlight))
            .await
    }
    async fn parse_expression_wv(
        &self,
        expr: String,
        flags: String,
        highlight: Boolean,
    ) -> error::Result<Dict> {
        self.call_fn_wv("nvim_parse_expression".into(), (expr, flags, highlight))
            .await
    }
    async fn open_win(
        &self,
        buffer: Buffer,
        enter: Boolean,
        config: impl Serialize,
    ) -> error::Result<Window> {
        self.call_fn("nvim_open_win".into(), (buffer, enter, config))
            .await
    }
    async fn open_win_wv(
        &self,
        buffer: Buffer,
        enter: Boolean,
        config: Dict,
    ) -> error::Result<Window> {
        self.call_fn_wv("nvim_open_win".into(), (buffer, enter, config))
            .await
    }
    async fn win_set_config(&self, window: Window, config: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_win_set_config".into(), (window, config))
            .await
    }
    async fn win_set_config_wv(&self, window: Window, config: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_win_set_config".into(), (window, config))
            .await
    }
    async fn win_get_config<D: Deserialize<'static>>(&self, window: Window) -> error::Result<D> {
        self.call_fn("nvim_win_get_config".into(), (window,)).await
    }
    async fn win_get_config_wv(&self, window: Window) -> error::Result<Dict> {
        self.call_fn_wv("nvim_win_get_config".into(), (window,))
            .await
    }
    async fn win_get_buf(&self, window: Window) -> error::Result<Buffer> {
        self.call_fn("nvim_win_get_buf".into(), (window,)).await
    }
    async fn win_set_buf(&self, window: Window, buffer: Buffer) -> error::Result<()> {
        self.call_fn("nvim_win_set_buf".into(), (window, buffer))
            .await
    }
    async fn win_get_cursor(&self, window: Window) -> error::Result<Vec<Integer>> {
        self.call_fn("nvim_win_get_cursor".into(), (window,)).await
    }
    async fn win_set_cursor(&self, window: Window, pos: &[Integer]) -> error::Result<()> {
        self.call_fn("nvim_win_set_cursor".into(), (window, pos))
            .await
    }
    async fn win_get_height(&self, window: Window) -> error::Result<Integer> {
        self.call_fn("nvim_win_get_height".into(), (window,)).await
    }
    async fn win_set_height(&self, window: Window, height: Integer) -> error::Result<()> {
        self.call_fn("nvim_win_set_height".into(), (window, height))
            .await
    }
    async fn win_get_width(&self, window: Window) -> error::Result<Integer> {
        self.call_fn("nvim_win_get_width".into(), (window,)).await
    }
    async fn win_set_width(&self, window: Window, width: Integer) -> error::Result<()> {
        self.call_fn("nvim_win_set_width".into(), (window, width))
            .await
    }
    async fn win_get_var<D: Deserialize<'static>>(
        &self,
        window: Window,
        name: &str,
    ) -> error::Result<D> {
        self.call_fn("nvim_win_get_var".into(), (window, name))
            .await
    }
    async fn win_get_var_wv(&self, window: Window, name: String) -> error::Result<Object> {
        self.call_fn_wv("nvim_win_get_var".into(), (window, name))
            .await
    }
    async fn win_set_var(
        &self,
        window: Window,
        name: &str,
        value: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_win_set_var".into(), (window, name, value))
            .await
    }
    async fn win_set_var_wv(
        &self,
        window: Window,
        name: String,
        value: Object,
    ) -> error::Result<()> {
        self.call_fn_wv("nvim_win_set_var".into(), (window, name, value))
            .await
    }
    async fn win_del_var(&self, window: Window, name: &str) -> error::Result<()> {
        self.call_fn("nvim_win_del_var".into(), (window, name))
            .await
    }
    async fn win_get_position(&self, window: Window) -> error::Result<Vec<Integer>> {
        self.call_fn("nvim_win_get_position".into(), (window,))
            .await
    }
    async fn win_get_tabpage(&self, window: Window) -> error::Result<Tabpage> {
        self.call_fn("nvim_win_get_tabpage".into(), (window,)).await
    }
    async fn win_get_number(&self, window: Window) -> error::Result<Integer> {
        self.call_fn("nvim_win_get_number".into(), (window,)).await
    }
    async fn win_is_valid(&self, window: Window) -> error::Result<Boolean> {
        self.call_fn("nvim_win_is_valid".into(), (window,)).await
    }
    async fn win_hide(&self, window: Window) -> error::Result<()> {
        self.call_fn("nvim_win_hide".into(), (window,)).await
    }
    async fn win_close(&self, window: Window, force: Boolean) -> error::Result<()> {
        self.call_fn("nvim_win_close".into(), (window, force)).await
    }
    async fn win_set_hl_ns(&self, window: Window, ns_id: Integer) -> error::Result<()> {
        self.call_fn("nvim_win_set_hl_ns".into(), (window, ns_id))
            .await
    }
    async fn win_text_height<D: Deserialize<'static>>(
        &self,
        window: Window,
        opts: impl Serialize,
    ) -> error::Result<D> {
        self.call_fn("nvim_win_text_height".into(), (window, opts))
            .await
    }
    async fn win_text_height_wv(&self, window: Window, opts: Dict) -> error::Result<Dict> {
        self.call_fn_wv("nvim_win_text_height".into(), (window, opts))
            .await
    }
}

pub trait NvimapiNr {
    fn call_fn_wv(
        &self,
        fn_name: String,
        args: impl crate::valueseq::ValueSeq,
    ) -> error::Result<()>;

    fn call_fn(&self, fn_name: &str, args: impl crate::valueseq::SerialSeq) -> error::Result<()>;
    fn get_autocmds(&self, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_get_autocmds".into(), (opts,))
    }
    fn get_autocmds_wv(&self, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_get_autocmds".into(), (opts,))
    }
    fn create_autocmd(&self, event: impl Serialize, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_create_autocmd".into(), (event, opts))
    }
    fn create_autocmd_wv(&self, event: Object, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_create_autocmd".into(), (event, opts))
    }
    fn del_autocmd(&self, id: Integer) -> error::Result<()> {
        self.call_fn("nvim_del_autocmd".into(), (id,))
    }
    fn clear_autocmds(&self, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_clear_autocmds".into(), (opts,))
    }
    fn clear_autocmds_wv(&self, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_clear_autocmds".into(), (opts,))
    }
    fn create_augroup(&self, name: &str, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_create_augroup".into(), (name, opts))
    }
    fn create_augroup_wv(&self, name: String, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_create_augroup".into(), (name, opts))
    }
    fn del_augroup_by_id(&self, id: Integer) -> error::Result<()> {
        self.call_fn("nvim_del_augroup_by_id".into(), (id,))
    }
    fn del_augroup_by_name(&self, name: &str) -> error::Result<()> {
        self.call_fn("nvim_del_augroup_by_name".into(), (name,))
    }
    fn exec_autocmds(&self, event: impl Serialize, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_exec_autocmds".into(), (event, opts))
    }
    fn exec_autocmds_wv(&self, event: Object, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_exec_autocmds".into(), (event, opts))
    }
    fn buf_line_count(&self, buffer: Buffer) -> error::Result<()> {
        self.call_fn("nvim_buf_line_count".into(), (buffer,))
    }
    fn buf_attach(
        &self,
        buffer: Buffer,
        send_buffer: Boolean,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_buf_attach".into(), (buffer, send_buffer, opts))
    }
    fn buf_attach_wv(&self, buffer: Buffer, send_buffer: Boolean, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_buf_attach".into(), (buffer, send_buffer, opts))
    }
    fn buf_detach(&self, buffer: Buffer) -> error::Result<()> {
        self.call_fn("nvim_buf_detach".into(), (buffer,))
    }
    fn buf_get_lines(
        &self,
        buffer: Buffer,
        start: Integer,
        end: Integer,
        strict_indexing: Boolean,
    ) -> error::Result<()> {
        self.call_fn(
            "nvim_buf_get_lines".into(),
            (buffer, start, end, strict_indexing),
        )
    }
    fn buf_set_lines(
        &self,
        buffer: Buffer,
        start: Integer,
        end: Integer,
        strict_indexing: Boolean,
        replacement: &[&str],
    ) -> error::Result<()> {
        self.call_fn(
            "nvim_buf_set_lines".into(),
            (buffer, start, end, strict_indexing, replacement),
        )
    }
    fn buf_set_text(
        &self,
        buffer: Buffer,
        start_row: Integer,
        start_col: Integer,
        end_row: Integer,
        end_col: Integer,
        replacement: &[&str],
    ) -> error::Result<()> {
        self.call_fn(
            "nvim_buf_set_text".into(),
            (buffer, start_row, start_col, end_row, end_col, replacement),
        )
    }
    fn buf_get_text(
        &self,
        buffer: Buffer,
        start_row: Integer,
        start_col: Integer,
        end_row: Integer,
        end_col: Integer,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn(
            "nvim_buf_get_text".into(),
            (buffer, start_row, start_col, end_row, end_col, opts),
        )
    }
    fn buf_get_text_wv(
        &self,
        buffer: Buffer,
        start_row: Integer,
        start_col: Integer,
        end_row: Integer,
        end_col: Integer,
        opts: Dict,
    ) -> error::Result<()> {
        self.call_fn_wv(
            "nvim_buf_get_text".into(),
            (buffer, start_row, start_col, end_row, end_col, opts),
        )
    }
    fn buf_get_offset(&self, buffer: Buffer, index: Integer) -> error::Result<()> {
        self.call_fn("nvim_buf_get_offset".into(), (buffer, index))
    }
    fn buf_get_var(&self, buffer: Buffer, name: &str) -> error::Result<()> {
        self.call_fn("nvim_buf_get_var".into(), (buffer, name))
    }
    fn buf_get_changedtick(&self, buffer: Buffer) -> error::Result<()> {
        self.call_fn("nvim_buf_get_changedtick".into(), (buffer,))
    }
    fn buf_get_keymap(&self, buffer: Buffer, mode: &str) -> error::Result<()> {
        self.call_fn("nvim_buf_get_keymap".into(), (buffer, mode))
    }
    fn buf_set_keymap(
        &self,
        buffer: Buffer,
        mode: &str,
        lhs: &str,
        rhs: &str,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_buf_set_keymap".into(), (buffer, mode, lhs, rhs, opts))
    }
    fn buf_set_keymap_wv(
        &self,
        buffer: Buffer,
        mode: String,
        lhs: String,
        rhs: String,
        opts: Dict,
    ) -> error::Result<()> {
        self.call_fn_wv("nvim_buf_set_keymap".into(), (buffer, mode, lhs, rhs, opts))
    }
    fn buf_del_keymap(&self, buffer: Buffer, mode: &str, lhs: &str) -> error::Result<()> {
        self.call_fn("nvim_buf_del_keymap".into(), (buffer, mode, lhs))
    }
    fn buf_set_var(&self, buffer: Buffer, name: &str, value: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_buf_set_var".into(), (buffer, name, value))
    }
    fn buf_set_var_wv(&self, buffer: Buffer, name: String, value: Object) -> error::Result<()> {
        self.call_fn_wv("nvim_buf_set_var".into(), (buffer, name, value))
    }
    fn buf_del_var(&self, buffer: Buffer, name: &str) -> error::Result<()> {
        self.call_fn("nvim_buf_del_var".into(), (buffer, name))
    }
    fn buf_get_name(&self, buffer: Buffer) -> error::Result<()> {
        self.call_fn("nvim_buf_get_name".into(), (buffer,))
    }
    fn buf_set_name(&self, buffer: Buffer, name: &str) -> error::Result<()> {
        self.call_fn("nvim_buf_set_name".into(), (buffer, name))
    }
    fn buf_is_loaded(&self, buffer: Buffer) -> error::Result<()> {
        self.call_fn("nvim_buf_is_loaded".into(), (buffer,))
    }
    fn buf_delete(&self, buffer: Buffer, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_buf_delete".into(), (buffer, opts))
    }
    fn buf_delete_wv(&self, buffer: Buffer, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_buf_delete".into(), (buffer, opts))
    }
    fn buf_is_valid(&self, buffer: Buffer) -> error::Result<()> {
        self.call_fn("nvim_buf_is_valid".into(), (buffer,))
    }
    fn buf_del_mark(&self, buffer: Buffer, name: &str) -> error::Result<()> {
        self.call_fn("nvim_buf_del_mark".into(), (buffer, name))
    }
    fn buf_set_mark(
        &self,
        buffer: Buffer,
        name: &str,
        line: Integer,
        col: Integer,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_buf_set_mark".into(), (buffer, name, line, col, opts))
    }
    fn buf_set_mark_wv(
        &self,
        buffer: Buffer,
        name: String,
        line: Integer,
        col: Integer,
        opts: Dict,
    ) -> error::Result<()> {
        self.call_fn_wv("nvim_buf_set_mark".into(), (buffer, name, line, col, opts))
    }
    fn buf_get_mark(&self, buffer: Buffer, name: &str) -> error::Result<()> {
        self.call_fn("nvim_buf_get_mark".into(), (buffer, name))
    }
    fn parse_cmd(&self, str: &str, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_parse_cmd".into(), (str, opts))
    }
    fn parse_cmd_wv(&self, str: String, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_parse_cmd".into(), (str, opts))
    }
    fn cmd(&self, cmd: impl Serialize, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_cmd".into(), (cmd, opts))
    }
    fn cmd_wv(&self, cmd: Dict, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_cmd".into(), (cmd, opts))
    }
    fn create_user_command(
        &self,
        name: &str,
        command: impl Serialize,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_create_user_command".into(), (name, command, opts))
    }
    fn create_user_command_wv(
        &self,
        name: String,
        command: Object,
        opts: Dict,
    ) -> error::Result<()> {
        self.call_fn_wv("nvim_create_user_command".into(), (name, command, opts))
    }
    fn del_user_command(&self, name: &str) -> error::Result<()> {
        self.call_fn("nvim_del_user_command".into(), (name,))
    }
    fn buf_create_user_command(
        &self,
        buffer: Buffer,
        name: &str,
        command: impl Serialize,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn(
            "nvim_buf_create_user_command".into(),
            (buffer, name, command, opts),
        )
    }
    fn buf_create_user_command_wv(
        &self,
        buffer: Buffer,
        name: String,
        command: Object,
        opts: Dict,
    ) -> error::Result<()> {
        self.call_fn_wv(
            "nvim_buf_create_user_command".into(),
            (buffer, name, command, opts),
        )
    }
    fn buf_del_user_command(&self, buffer: Buffer, name: &str) -> error::Result<()> {
        self.call_fn("nvim_buf_del_user_command".into(), (buffer, name))
    }
    fn get_commands(&self, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_get_commands".into(), (opts,))
    }
    fn get_commands_wv(&self, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_get_commands".into(), (opts,))
    }
    fn buf_get_commands(&self, buffer: Buffer, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_buf_get_commands".into(), (buffer, opts))
    }
    fn buf_get_commands_wv(&self, buffer: Buffer, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_buf_get_commands".into(), (buffer, opts))
    }
    fn create_namespace(&self, name: &str) -> error::Result<()> {
        self.call_fn("nvim_create_namespace".into(), (name,))
    }
    fn get_namespaces(&self) -> error::Result<()> {
        self.call_fn("nvim_get_namespaces".into(), [(); 0])
    }
    fn buf_get_extmark_by_id(
        &self,
        buffer: Buffer,
        ns_id: Integer,
        id: Integer,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn(
            "nvim_buf_get_extmark_by_id".into(),
            (buffer, ns_id, id, opts),
        )
    }
    fn buf_get_extmark_by_id_wv(
        &self,
        buffer: Buffer,
        ns_id: Integer,
        id: Integer,
        opts: Dict,
    ) -> error::Result<()> {
        self.call_fn_wv(
            "nvim_buf_get_extmark_by_id".into(),
            (buffer, ns_id, id, opts),
        )
    }
    fn buf_get_extmarks(
        &self,
        buffer: Buffer,
        ns_id: Integer,
        start: impl Serialize,
        end: impl Serialize,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn(
            "nvim_buf_get_extmarks".into(),
            (buffer, ns_id, start, end, opts),
        )
    }
    fn buf_get_extmarks_wv(
        &self,
        buffer: Buffer,
        ns_id: Integer,
        start: Object,
        end: Object,
        opts: Dict,
    ) -> error::Result<()> {
        self.call_fn_wv(
            "nvim_buf_get_extmarks".into(),
            (buffer, ns_id, start, end, opts),
        )
    }
    fn buf_set_extmark(
        &self,
        buffer: Buffer,
        ns_id: Integer,
        line: Integer,
        col: Integer,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn(
            "nvim_buf_set_extmark".into(),
            (buffer, ns_id, line, col, opts),
        )
    }
    fn buf_set_extmark_wv(
        &self,
        buffer: Buffer,
        ns_id: Integer,
        line: Integer,
        col: Integer,
        opts: Dict,
    ) -> error::Result<()> {
        self.call_fn_wv(
            "nvim_buf_set_extmark".into(),
            (buffer, ns_id, line, col, opts),
        )
    }
    fn buf_del_extmark(&self, buffer: Buffer, ns_id: Integer, id: Integer) -> error::Result<()> {
        self.call_fn("nvim_buf_del_extmark".into(), (buffer, ns_id, id))
    }
    fn buf_clear_namespace(
        &self,
        buffer: Buffer,
        ns_id: Integer,
        line_start: Integer,
        line_end: Integer,
    ) -> error::Result<()> {
        self.call_fn(
            "nvim_buf_clear_namespace".into(),
            (buffer, ns_id, line_start, line_end),
        )
    }
    fn set_decoration_provider(&self, ns_id: Integer, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_set_decoration_provider".into(), (ns_id, opts))
    }
    fn set_decoration_provider_wv(&self, ns_id: Integer, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_set_decoration_provider".into(), (ns_id, opts))
    }
    fn get_option_value(&self, name: &str, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_get_option_value".into(), (name, opts))
    }
    fn get_option_value_wv(&self, name: String, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_get_option_value".into(), (name, opts))
    }
    fn set_option_value(
        &self,
        name: &str,
        value: impl Serialize,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_set_option_value".into(), (name, value, opts))
    }
    fn set_option_value_wv(&self, name: String, value: Object, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_set_option_value".into(), (name, value, opts))
    }
    fn get_all_options_info(&self) -> error::Result<()> {
        self.call_fn("nvim_get_all_options_info".into(), [(); 0])
    }
    fn get_option_info2(&self, name: &str, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_get_option_info2".into(), (name, opts))
    }
    fn get_option_info2_wv(&self, name: String, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_get_option_info2".into(), (name, opts))
    }
    fn tabpage_list_wins(&self, tabpage: Tabpage) -> error::Result<()> {
        self.call_fn("nvim_tabpage_list_wins".into(), (tabpage,))
    }
    fn tabpage_get_var(&self, tabpage: Tabpage, name: &str) -> error::Result<()> {
        self.call_fn("nvim_tabpage_get_var".into(), (tabpage, name))
    }
    fn tabpage_set_var(
        &self,
        tabpage: Tabpage,
        name: &str,
        value: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_tabpage_set_var".into(), (tabpage, name, value))
    }
    fn tabpage_set_var_wv(
        &self,
        tabpage: Tabpage,
        name: String,
        value: Object,
    ) -> error::Result<()> {
        self.call_fn_wv("nvim_tabpage_set_var".into(), (tabpage, name, value))
    }
    fn tabpage_del_var(&self, tabpage: Tabpage, name: &str) -> error::Result<()> {
        self.call_fn("nvim_tabpage_del_var".into(), (tabpage, name))
    }
    fn tabpage_get_win(&self, tabpage: Tabpage) -> error::Result<()> {
        self.call_fn("nvim_tabpage_get_win".into(), (tabpage,))
    }
    fn tabpage_set_win(&self, tabpage: Tabpage, win: Window) -> error::Result<()> {
        self.call_fn("nvim_tabpage_set_win".into(), (tabpage, win))
    }
    fn tabpage_get_number(&self, tabpage: Tabpage) -> error::Result<()> {
        self.call_fn("nvim_tabpage_get_number".into(), (tabpage,))
    }
    fn tabpage_is_valid(&self, tabpage: Tabpage) -> error::Result<()> {
        self.call_fn("nvim_tabpage_is_valid".into(), (tabpage,))
    }
    fn ui_attach(
        &self,
        width: Integer,
        height: Integer,
        options: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_ui_attach".into(), (width, height, options))
    }
    fn ui_attach_wv(&self, width: Integer, height: Integer, options: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_ui_attach".into(), (width, height, options))
    }
    fn ui_set_focus(&self, gained: Boolean) -> error::Result<()> {
        self.call_fn("nvim_ui_set_focus".into(), (gained,))
    }
    fn ui_detach(&self) -> error::Result<()> {
        self.call_fn("nvim_ui_detach".into(), [(); 0])
    }
    fn ui_try_resize(&self, width: Integer, height: Integer) -> error::Result<()> {
        self.call_fn("nvim_ui_try_resize".into(), (width, height))
    }
    fn ui_set_option(&self, name: &str, value: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_ui_set_option".into(), (name, value))
    }
    fn ui_set_option_wv(&self, name: String, value: Object) -> error::Result<()> {
        self.call_fn_wv("nvim_ui_set_option".into(), (name, value))
    }
    fn ui_try_resize_grid(
        &self,
        grid: Integer,
        width: Integer,
        height: Integer,
    ) -> error::Result<()> {
        self.call_fn("nvim_ui_try_resize_grid".into(), (grid, width, height))
    }
    fn ui_pum_set_height(&self, height: Integer) -> error::Result<()> {
        self.call_fn("nvim_ui_pum_set_height".into(), (height,))
    }
    fn ui_pum_set_bounds(
        &self,
        width: Float,
        height: Float,
        row: Float,
        col: Float,
    ) -> error::Result<()> {
        self.call_fn("nvim_ui_pum_set_bounds".into(), (width, height, row, col))
    }
    fn ui_term_event(&self, event: &str, value: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_ui_term_event".into(), (event, value))
    }
    fn ui_term_event_wv(&self, event: String, value: Object) -> error::Result<()> {
        self.call_fn_wv("nvim_ui_term_event".into(), (event, value))
    }
    fn get_hl_id_by_name(&self, name: &str) -> error::Result<()> {
        self.call_fn("nvim_get_hl_id_by_name".into(), (name,))
    }
    fn get_hl(&self, ns_id: Integer, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_get_hl".into(), (ns_id, opts))
    }
    fn get_hl_wv(&self, ns_id: Integer, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_get_hl".into(), (ns_id, opts))
    }
    fn set_hl(&self, ns_id: Integer, name: &str, val: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_set_hl".into(), (ns_id, name, val))
    }
    fn set_hl_wv(&self, ns_id: Integer, name: String, val: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_set_hl".into(), (ns_id, name, val))
    }
    fn get_hl_ns(&self, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_get_hl_ns".into(), (opts,))
    }
    fn get_hl_ns_wv(&self, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_get_hl_ns".into(), (opts,))
    }
    fn set_hl_ns(&self, ns_id: Integer) -> error::Result<()> {
        self.call_fn("nvim_set_hl_ns".into(), (ns_id,))
    }
    fn set_hl_ns_fast(&self, ns_id: Integer) -> error::Result<()> {
        self.call_fn("nvim_set_hl_ns_fast".into(), (ns_id,))
    }
    fn feedkeys(&self, keys: &str, mode: &str, escape_ks: Boolean) -> error::Result<()> {
        self.call_fn("nvim_feedkeys".into(), (keys, mode, escape_ks))
    }
    fn input(&self, keys: &str) -> error::Result<()> {
        self.call_fn("nvim_input".into(), (keys,))
    }
    fn input_mouse(
        &self,
        button: &str,
        action: &str,
        modifier: &str,
        grid: Integer,
        row: Integer,
        col: Integer,
    ) -> error::Result<()> {
        self.call_fn(
            "nvim_input_mouse".into(),
            (button, action, modifier, grid, row, col),
        )
    }
    fn replace_termcodes(
        &self,
        str: &str,
        from_part: Boolean,
        do_lt: Boolean,
        special: Boolean,
    ) -> error::Result<()> {
        self.call_fn(
            "nvim_replace_termcodes".into(),
            (str, from_part, do_lt, special),
        )
    }
    fn exec_lua(&self, code: &str, args: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_exec_lua".into(), (code, args))
    }
    fn exec_lua_wv(&self, code: String, args: Array) -> error::Result<()> {
        self.call_fn_wv("nvim_exec_lua".into(), (code, args))
    }
    fn strwidth(&self, text: &str) -> error::Result<()> {
        self.call_fn("nvim_strwidth".into(), (text,))
    }
    fn list_runtime_paths(&self) -> error::Result<()> {
        self.call_fn("nvim_list_runtime_paths".into(), [(); 0])
    }
    fn get_runtime_file(&self, name: &str, all: Boolean) -> error::Result<()> {
        self.call_fn("nvim_get_runtime_file".into(), (name, all))
    }
    fn set_current_dir(&self, dir: &str) -> error::Result<()> {
        self.call_fn("nvim_set_current_dir".into(), (dir,))
    }
    fn get_current_line(&self) -> error::Result<()> {
        self.call_fn("nvim_get_current_line".into(), [(); 0])
    }
    fn set_current_line(&self, line: &str) -> error::Result<()> {
        self.call_fn("nvim_set_current_line".into(), (line,))
    }
    fn del_current_line(&self) -> error::Result<()> {
        self.call_fn("nvim_del_current_line".into(), [(); 0])
    }
    fn get_var(&self, name: &str) -> error::Result<()> {
        self.call_fn("nvim_get_var".into(), (name,))
    }
    fn set_var(&self, name: &str, value: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_set_var".into(), (name, value))
    }
    fn set_var_wv(&self, name: String, value: Object) -> error::Result<()> {
        self.call_fn_wv("nvim_set_var".into(), (name, value))
    }
    fn del_var(&self, name: &str) -> error::Result<()> {
        self.call_fn("nvim_del_var".into(), (name,))
    }
    fn get_vvar(&self, name: &str) -> error::Result<()> {
        self.call_fn("nvim_get_vvar".into(), (name,))
    }
    fn set_vvar(&self, name: &str, value: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_set_vvar".into(), (name, value))
    }
    fn set_vvar_wv(&self, name: String, value: Object) -> error::Result<()> {
        self.call_fn_wv("nvim_set_vvar".into(), (name, value))
    }
    fn echo(
        &self,
        chunks: impl Serialize,
        history: Boolean,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_echo".into(), (chunks, history, opts))
    }
    fn echo_wv(&self, chunks: Array, history: Boolean, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_echo".into(), (chunks, history, opts))
    }
    fn list_bufs(&self) -> error::Result<()> {
        self.call_fn("nvim_list_bufs".into(), [(); 0])
    }
    fn get_current_buf(&self) -> error::Result<()> {
        self.call_fn("nvim_get_current_buf".into(), [(); 0])
    }
    fn set_current_buf(&self, buffer: Buffer) -> error::Result<()> {
        self.call_fn("nvim_set_current_buf".into(), (buffer,))
    }
    fn list_wins(&self) -> error::Result<()> {
        self.call_fn("nvim_list_wins".into(), [(); 0])
    }
    fn get_current_win(&self) -> error::Result<()> {
        self.call_fn("nvim_get_current_win".into(), [(); 0])
    }
    fn set_current_win(&self, window: Window) -> error::Result<()> {
        self.call_fn("nvim_set_current_win".into(), (window,))
    }
    fn create_buf(&self, listed: Boolean, scratch: Boolean) -> error::Result<()> {
        self.call_fn("nvim_create_buf".into(), (listed, scratch))
    }
    fn open_term(&self, buffer: Buffer, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_open_term".into(), (buffer, opts))
    }
    fn open_term_wv(&self, buffer: Buffer, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_open_term".into(), (buffer, opts))
    }
    fn chan_send(&self, chan: Integer, data: &str) -> error::Result<()> {
        self.call_fn("nvim_chan_send".into(), (chan, data))
    }
    fn list_tabpages(&self) -> error::Result<()> {
        self.call_fn("nvim_list_tabpages".into(), [(); 0])
    }
    fn get_current_tabpage(&self) -> error::Result<()> {
        self.call_fn("nvim_get_current_tabpage".into(), [(); 0])
    }
    fn set_current_tabpage(&self, tabpage: Tabpage) -> error::Result<()> {
        self.call_fn("nvim_set_current_tabpage".into(), (tabpage,))
    }
    fn paste(&self, data: &str, crlf: Boolean, phase: Integer) -> error::Result<()> {
        self.call_fn("nvim_paste".into(), (data, crlf, phase))
    }
    fn put(
        &self,
        lines: &[&str],
        type_: &str,
        after: Boolean,
        follow: Boolean,
    ) -> error::Result<()> {
        self.call_fn("nvim_put".into(), (lines, type_, after, follow))
    }
    fn get_color_by_name(&self, name: &str) -> error::Result<()> {
        self.call_fn("nvim_get_color_by_name".into(), (name,))
    }
    fn get_color_map(&self) -> error::Result<()> {
        self.call_fn("nvim_get_color_map".into(), [(); 0])
    }
    fn get_context(&self, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_get_context".into(), (opts,))
    }
    fn get_context_wv(&self, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_get_context".into(), (opts,))
    }
    fn load_context(&self, dict: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_load_context".into(), (dict,))
    }
    fn load_context_wv(&self, dict: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_load_context".into(), (dict,))
    }
    fn get_mode(&self) -> error::Result<()> {
        self.call_fn("nvim_get_mode".into(), [(); 0])
    }
    fn get_keymap(&self, mode: &str) -> error::Result<()> {
        self.call_fn("nvim_get_keymap".into(), (mode,))
    }
    fn set_keymap(
        &self,
        mode: &str,
        lhs: &str,
        rhs: &str,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_set_keymap".into(), (mode, lhs, rhs, opts))
    }
    fn set_keymap_wv(
        &self,
        mode: String,
        lhs: String,
        rhs: String,
        opts: Dict,
    ) -> error::Result<()> {
        self.call_fn_wv("nvim_set_keymap".into(), (mode, lhs, rhs, opts))
    }
    fn del_keymap(&self, mode: &str, lhs: &str) -> error::Result<()> {
        self.call_fn("nvim_del_keymap".into(), (mode, lhs))
    }
    fn get_api_info(&self) -> error::Result<()> {
        self.call_fn("nvim_get_api_info".into(), [(); 0])
    }
    fn set_client_info(
        &self,
        name: &str,
        version: impl Serialize,
        type_: &str,
        methods: impl Serialize,
        attributes: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn(
            "nvim_set_client_info".into(),
            (name, version, type_, methods, attributes),
        )
    }
    fn set_client_info_wv(
        &self,
        name: String,
        version: Dict,
        type_: String,
        methods: Dict,
        attributes: Dict,
    ) -> error::Result<()> {
        self.call_fn_wv(
            "nvim_set_client_info".into(),
            (name, version, type_, methods, attributes),
        )
    }
    fn get_chan_info(&self, chan: Integer) -> error::Result<()> {
        self.call_fn("nvim_get_chan_info".into(), (chan,))
    }
    fn list_chans(&self) -> error::Result<()> {
        self.call_fn("nvim_list_chans".into(), [(); 0])
    }
    fn list_uis(&self) -> error::Result<()> {
        self.call_fn("nvim_list_uis".into(), [(); 0])
    }
    fn get_proc_children(&self, pid: Integer) -> error::Result<()> {
        self.call_fn("nvim_get_proc_children".into(), (pid,))
    }
    fn get_proc(&self, pid: Integer) -> error::Result<()> {
        self.call_fn("nvim_get_proc".into(), (pid,))
    }
    fn select_popupmenu_item(
        &self,
        item: Integer,
        insert: Boolean,
        finish: Boolean,
        opts: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn(
            "nvim_select_popupmenu_item".into(),
            (item, insert, finish, opts),
        )
    }
    fn select_popupmenu_item_wv(
        &self,
        item: Integer,
        insert: Boolean,
        finish: Boolean,
        opts: Dict,
    ) -> error::Result<()> {
        self.call_fn_wv(
            "nvim_select_popupmenu_item".into(),
            (item, insert, finish, opts),
        )
    }
    fn del_mark(&self, name: &str) -> error::Result<()> {
        self.call_fn("nvim_del_mark".into(), (name,))
    }
    fn get_mark(&self, name: &str, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_get_mark".into(), (name, opts))
    }
    fn get_mark_wv(&self, name: String, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_get_mark".into(), (name, opts))
    }
    fn eval_statusline(&self, str: &str, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_eval_statusline".into(), (str, opts))
    }
    fn eval_statusline_wv(&self, str: String, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_eval_statusline".into(), (str, opts))
    }
    fn exec2(&self, src: &str, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_exec2".into(), (src, opts))
    }
    fn exec2_wv(&self, src: String, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_exec2".into(), (src, opts))
    }
    fn command(&self, command: &str) -> error::Result<()> {
        self.call_fn("nvim_command".into(), (command,))
    }
    fn eval(&self, expr: &str) -> error::Result<()> {
        self.call_fn("nvim_eval".into(), (expr,))
    }
    fn call_function(&self, fn_: &str, args: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_call_function".into(), (fn_, args))
    }
    fn call_function_wv(&self, fn_: String, args: Array) -> error::Result<()> {
        self.call_fn_wv("nvim_call_function".into(), (fn_, args))
    }
    fn call_dict_function(
        &self,
        dict: impl Serialize,
        fn_: &str,
        args: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_call_dict_function".into(), (dict, fn_, args))
    }
    fn call_dict_function_wv(&self, dict: Object, fn_: String, args: Array) -> error::Result<()> {
        self.call_fn_wv("nvim_call_dict_function".into(), (dict, fn_, args))
    }
    fn parse_expression(&self, expr: &str, flags: &str, highlight: Boolean) -> error::Result<()> {
        self.call_fn("nvim_parse_expression".into(), (expr, flags, highlight))
    }
    fn open_win(
        &self,
        buffer: Buffer,
        enter: Boolean,
        config: impl Serialize,
    ) -> error::Result<()> {
        self.call_fn("nvim_open_win".into(), (buffer, enter, config))
    }
    fn open_win_wv(&self, buffer: Buffer, enter: Boolean, config: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_open_win".into(), (buffer, enter, config))
    }
    fn win_set_config(&self, window: Window, config: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_win_set_config".into(), (window, config))
    }
    fn win_set_config_wv(&self, window: Window, config: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_win_set_config".into(), (window, config))
    }
    fn win_get_config(&self, window: Window) -> error::Result<()> {
        self.call_fn("nvim_win_get_config".into(), (window,))
    }
    fn win_get_buf(&self, window: Window) -> error::Result<()> {
        self.call_fn("nvim_win_get_buf".into(), (window,))
    }
    fn win_set_buf(&self, window: Window, buffer: Buffer) -> error::Result<()> {
        self.call_fn("nvim_win_set_buf".into(), (window, buffer))
    }
    fn win_get_cursor(&self, window: Window) -> error::Result<()> {
        self.call_fn("nvim_win_get_cursor".into(), (window,))
    }
    fn win_set_cursor(&self, window: Window, pos: &[Integer]) -> error::Result<()> {
        self.call_fn("nvim_win_set_cursor".into(), (window, pos))
    }
    fn win_get_height(&self, window: Window) -> error::Result<()> {
        self.call_fn("nvim_win_get_height".into(), (window,))
    }
    fn win_set_height(&self, window: Window, height: Integer) -> error::Result<()> {
        self.call_fn("nvim_win_set_height".into(), (window, height))
    }
    fn win_get_width(&self, window: Window) -> error::Result<()> {
        self.call_fn("nvim_win_get_width".into(), (window,))
    }
    fn win_set_width(&self, window: Window, width: Integer) -> error::Result<()> {
        self.call_fn("nvim_win_set_width".into(), (window, width))
    }
    fn win_get_var(&self, window: Window, name: &str) -> error::Result<()> {
        self.call_fn("nvim_win_get_var".into(), (window, name))
    }
    fn win_set_var(&self, window: Window, name: &str, value: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_win_set_var".into(), (window, name, value))
    }
    fn win_set_var_wv(&self, window: Window, name: String, value: Object) -> error::Result<()> {
        self.call_fn_wv("nvim_win_set_var".into(), (window, name, value))
    }
    fn win_del_var(&self, window: Window, name: &str) -> error::Result<()> {
        self.call_fn("nvim_win_del_var".into(), (window, name))
    }
    fn win_get_position(&self, window: Window) -> error::Result<()> {
        self.call_fn("nvim_win_get_position".into(), (window,))
    }
    fn win_get_tabpage(&self, window: Window) -> error::Result<()> {
        self.call_fn("nvim_win_get_tabpage".into(), (window,))
    }
    fn win_get_number(&self, window: Window) -> error::Result<()> {
        self.call_fn("nvim_win_get_number".into(), (window,))
    }
    fn win_is_valid(&self, window: Window) -> error::Result<()> {
        self.call_fn("nvim_win_is_valid".into(), (window,))
    }
    fn win_hide(&self, window: Window) -> error::Result<()> {
        self.call_fn("nvim_win_hide".into(), (window,))
    }
    fn win_close(&self, window: Window, force: Boolean) -> error::Result<()> {
        self.call_fn("nvim_win_close".into(), (window, force))
    }
    fn win_set_hl_ns(&self, window: Window, ns_id: Integer) -> error::Result<()> {
        self.call_fn("nvim_win_set_hl_ns".into(), (window, ns_id))
    }
    fn win_text_height(&self, window: Window, opts: impl Serialize) -> error::Result<()> {
        self.call_fn("nvim_win_text_height".into(), (window, opts))
    }
    fn win_text_height_wv(&self, window: Window, opts: Dict) -> error::Result<()> {
        self.call_fn_wv("nvim_win_text_height".into(), (window, opts))
    }
}
pub mod uievent {
    pub use super::*;
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct ModeInfoSet {
        pub enabled: Boolean,
        pub cursor_styles: Array,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct UpdateMenu {}
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct BusyStart {}
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct BusyStop {}
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct MouseOn {}
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct MouseOff {}
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct ModeChange {
        pub mode: String,
        pub mode_idx: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct Bell {}
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct VisualBell {}
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct Flush {}
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct Suspend {}
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct SetTitle {
        pub title: String,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct SetIcon {
        pub icon: String,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct Screenshot {
        pub path: String,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct OptionSet {
        pub name: String,
        pub value: Object,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct Chdir {
        pub path: String,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct UpdateFg {
        pub fg: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct UpdateBg {
        pub bg: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct UpdateSp {
        pub sp: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct Resize {
        pub width: Integer,
        pub height: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct Clear {}
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct EolClear {}
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct CursorGoto {
        pub row: Integer,
        pub col: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct HighlightSet {
        pub attrs: Dict,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct Put {
        pub str: String,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct SetScrollRegion {
        pub top: Integer,
        pub bot: Integer,
        pub left: Integer,
        pub right: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct Scroll {
        pub count: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct DefaultColorsSet {
        pub rgb_fg: Integer,
        pub rgb_bg: Integer,
        pub rgb_sp: Integer,
        pub cterm_fg: Integer,
        pub cterm_bg: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct HlAttrDefine {
        pub id: Integer,
        pub rgb_attrs: Dict,
        pub cterm_attrs: Dict,
        pub info: Array,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct HlGroupSet {
        pub name: String,
        pub id: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct GridResize {
        pub grid: Integer,
        pub width: Integer,
        pub height: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct GridClear {
        pub grid: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct GridCursorGoto {
        pub grid: Integer,
        pub row: Integer,
        pub col: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct GridLine {
        pub grid: Integer,
        pub row: Integer,
        pub col_start: Integer,
        pub data: Array,
        pub wrap: Boolean,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct GridScroll {
        pub grid: Integer,
        pub top: Integer,
        pub bot: Integer,
        pub left: Integer,
        pub right: Integer,
        pub rows: Integer,
        pub cols: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct GridDestroy {
        pub grid: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct WinPos {
        pub grid: Integer,
        pub win: Window,
        pub startrow: Integer,
        pub startcol: Integer,
        pub width: Integer,
        pub height: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
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
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct WinExternalPos {
        pub grid: Integer,
        pub win: Window,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct WinHide {
        pub grid: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct WinClose {
        pub grid: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct MsgSetPos {
        pub grid: Integer,
        pub row: Integer,
        pub scrolled: Boolean,
        pub sep_char: String,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
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
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct WinViewportMargins {
        pub grid: Integer,
        pub win: Window,
        pub top: Integer,
        pub bottom: Integer,
        pub left: Integer,
        pub right: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct WinExtmark {
        pub grid: Integer,
        pub win: Window,
        pub ns_id: Integer,
        pub mark_id: Integer,
        pub row: Integer,
        pub col: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct PopupmenuShow {
        pub items: Array,
        pub selected: Integer,
        pub row: Integer,
        pub col: Integer,
        pub grid: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct PopupmenuHide {}
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct PopupmenuSelect {
        pub selected: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct TablineUpdate {
        pub current: Tabpage,
        pub tabs: Array,
        pub current_buffer: Buffer,
        pub buffers: Array,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct CmdlineShow {
        pub content: Array,
        pub pos: Integer,
        pub firstc: String,
        pub prompt: String,
        pub indent: Integer,
        pub level: Integer,
        pub hl_id: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct CmdlinePos {
        pub pos: Integer,
        pub level: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct CmdlineSpecialChar {
        pub c: String,
        pub shift: Boolean,
        pub level: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct CmdlineHide {
        pub level: Integer,
        pub abort: Boolean,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct CmdlineBlockShow {
        pub lines: Array,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct CmdlineBlockAppend {
        pub lines: Array,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct CmdlineBlockHide {}
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct WildmenuShow {
        pub items: Array,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct WildmenuSelect {
        pub selected: Integer,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct WildmenuHide {}
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct MsgShow {
        pub kind: String,
        pub content: Array,
        pub replace_last: Boolean,
        pub history: Boolean,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct MsgClear {}
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct MsgShowcmd {
        pub content: Array,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct MsgShowmode {
        pub content: Array,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct MsgRuler {
        pub content: Array,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct MsgHistoryShow {
        pub entries: Array,
    }
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct MsgHistoryClear {}
    #[derive(Deserialize, serde::Serialize, Debug)]
    pub struct ErrorExit {
        pub status: Integer,
    }
    #[derive(Debug)]
    pub enum UiEvent {
        ModeInfoSet(Vec<ModeInfoSet>),
        UpdateMenu(Vec<UpdateMenu>),
        BusyStart(Vec<BusyStart>),
        BusyStop(Vec<BusyStop>),
        MouseOn(Vec<MouseOn>),
        MouseOff(Vec<MouseOff>),
        ModeChange(Vec<ModeChange>),
        Bell(Vec<Bell>),
        VisualBell(Vec<VisualBell>),
        Flush(Vec<Flush>),
        Suspend(Vec<Suspend>),
        SetTitle(Vec<SetTitle>),
        SetIcon(Vec<SetIcon>),
        Screenshot(Vec<Screenshot>),
        OptionSet(Vec<OptionSet>),
        Chdir(Vec<Chdir>),
        UpdateFg(Vec<UpdateFg>),
        UpdateBg(Vec<UpdateBg>),
        UpdateSp(Vec<UpdateSp>),
        Resize(Vec<Resize>),
        Clear(Vec<Clear>),
        EolClear(Vec<EolClear>),
        CursorGoto(Vec<CursorGoto>),
        HighlightSet(Vec<HighlightSet>),
        Put(Vec<Put>),
        SetScrollRegion(Vec<SetScrollRegion>),
        Scroll(Vec<Scroll>),
        DefaultColorsSet(Vec<DefaultColorsSet>),
        HlAttrDefine(Vec<HlAttrDefine>),
        HlGroupSet(Vec<HlGroupSet>),
        GridResize(Vec<GridResize>),
        GridClear(Vec<GridClear>),
        GridCursorGoto(Vec<GridCursorGoto>),
        GridLine(Vec<GridLine>),
        GridScroll(Vec<GridScroll>),
        GridDestroy(Vec<GridDestroy>),
        WinPos(Vec<WinPos>),
        WinFloatPos(Vec<WinFloatPos>),
        WinExternalPos(Vec<WinExternalPos>),
        WinHide(Vec<WinHide>),
        WinClose(Vec<WinClose>),
        MsgSetPos(Vec<MsgSetPos>),
        WinViewport(Vec<WinViewport>),
        WinViewportMargins(Vec<WinViewportMargins>),
        WinExtmark(Vec<WinExtmark>),
        PopupmenuShow(Vec<PopupmenuShow>),
        PopupmenuHide(Vec<PopupmenuHide>),
        PopupmenuSelect(Vec<PopupmenuSelect>),
        TablineUpdate(Vec<TablineUpdate>),
        CmdlineShow(Vec<CmdlineShow>),
        CmdlinePos(Vec<CmdlinePos>),
        CmdlineSpecialChar(Vec<CmdlineSpecialChar>),
        CmdlineHide(Vec<CmdlineHide>),
        CmdlineBlockShow(Vec<CmdlineBlockShow>),
        CmdlineBlockAppend(Vec<CmdlineBlockAppend>),
        CmdlineBlockHide(Vec<CmdlineBlockHide>),
        WildmenuShow(Vec<WildmenuShow>),
        WildmenuSelect(Vec<WildmenuSelect>),
        WildmenuHide(Vec<WildmenuHide>),
        MsgShow(Vec<MsgShow>),
        MsgClear(Vec<MsgClear>),
        MsgShowcmd(Vec<MsgShowcmd>),
        MsgShowmode(Vec<MsgShowmode>),
        MsgRuler(Vec<MsgRuler>),
        MsgHistoryShow(Vec<MsgHistoryShow>),
        MsgHistoryClear(Vec<MsgHistoryClear>),
        ErrorExit(Vec<ErrorExit>),
        Unknown(Box<(String, Value)>),
    }

    impl UiEvent {
        pub fn name(&self) -> &'static str {
            match self {
                Self::ModeInfoSet(_) => "mode_info_set",
                Self::UpdateMenu(_) => "update_menu",
                Self::BusyStart(_) => "busy_start",
                Self::BusyStop(_) => "busy_stop",
                Self::MouseOn(_) => "mouse_on",
                Self::MouseOff(_) => "mouse_off",
                Self::ModeChange(_) => "mode_change",
                Self::Bell(_) => "bell",
                Self::VisualBell(_) => "visual_bell",
                Self::Flush(_) => "flush",
                Self::Suspend(_) => "suspend",
                Self::SetTitle(_) => "set_title",
                Self::SetIcon(_) => "set_icon",
                Self::Screenshot(_) => "screenshot",
                Self::OptionSet(_) => "option_set",
                Self::Chdir(_) => "chdir",
                Self::UpdateFg(_) => "update_fg",
                Self::UpdateBg(_) => "update_bg",
                Self::UpdateSp(_) => "update_sp",
                Self::Resize(_) => "resize",
                Self::Clear(_) => "clear",
                Self::EolClear(_) => "eol_clear",
                Self::CursorGoto(_) => "cursor_goto",
                Self::HighlightSet(_) => "highlight_set",
                Self::Put(_) => "put",
                Self::SetScrollRegion(_) => "set_scroll_region",
                Self::Scroll(_) => "scroll",
                Self::DefaultColorsSet(_) => "default_colors_set",
                Self::HlAttrDefine(_) => "hl_attr_define",
                Self::HlGroupSet(_) => "hl_group_set",
                Self::GridResize(_) => "grid_resize",
                Self::GridClear(_) => "grid_clear",
                Self::GridCursorGoto(_) => "grid_cursor_goto",
                Self::GridLine(_) => "grid_line",
                Self::GridScroll(_) => "grid_scroll",
                Self::GridDestroy(_) => "grid_destroy",
                Self::WinPos(_) => "win_pos",
                Self::WinFloatPos(_) => "win_float_pos",
                Self::WinExternalPos(_) => "win_external_pos",
                Self::WinHide(_) => "win_hide",
                Self::WinClose(_) => "win_close",
                Self::MsgSetPos(_) => "msg_set_pos",
                Self::WinViewport(_) => "win_viewport",
                Self::WinViewportMargins(_) => "win_viewport_margins",
                Self::WinExtmark(_) => "win_extmark",
                Self::PopupmenuShow(_) => "popupmenu_show",
                Self::PopupmenuHide(_) => "popupmenu_hide",
                Self::PopupmenuSelect(_) => "popupmenu_select",
                Self::TablineUpdate(_) => "tabline_update",
                Self::CmdlineShow(_) => "cmdline_show",
                Self::CmdlinePos(_) => "cmdline_pos",
                Self::CmdlineSpecialChar(_) => "cmdline_special_char",
                Self::CmdlineHide(_) => "cmdline_hide",
                Self::CmdlineBlockShow(_) => "cmdline_block_show",
                Self::CmdlineBlockAppend(_) => "cmdline_block_append",
                Self::CmdlineBlockHide(_) => "cmdline_block_hide",
                Self::WildmenuShow(_) => "wildmenu_show",
                Self::WildmenuSelect(_) => "wildmenu_select",
                Self::WildmenuHide(_) => "wildmenu_hide",
                Self::MsgShow(_) => "msg_show",
                Self::MsgClear(_) => "msg_clear",
                Self::MsgShowcmd(_) => "msg_showcmd",
                Self::MsgShowmode(_) => "msg_showmode",
                Self::MsgRuler(_) => "msg_ruler",
                Self::MsgHistoryShow(_) => "msg_history_show",
                Self::MsgHistoryClear(_) => "msg_history_clear",
                Self::ErrorExit(_) => "error_exit",
                Self::Unknown(_) => "unknown",
            }
        }
    }
    impl<'de> Deserialize<'de> for UiEvent {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
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
                    use serde::de::Error as DError;
                    let msg = "missing element, expected 2 elements";
                    let Some(event_name) = seq.next_element::<String>()? else {
                        return Err(DError::custom(msg));
                    };
                    match event_name.as_str() {
                        "mode_info_set" => {
                            let inner = Vec::<ModeInfoSet>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::ModeInfoSet(inner));
                        }
                        "update_menu" => {
                            let inner = Vec::<UpdateMenu>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::UpdateMenu(inner));
                        }
                        "busy_start" => {
                            let inner = Vec::<BusyStart>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::BusyStart(inner));
                        }
                        "busy_stop" => {
                            let inner = Vec::<BusyStop>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::BusyStop(inner));
                        }
                        "mouse_on" => {
                            let inner = Vec::<MouseOn>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::MouseOn(inner));
                        }
                        "mouse_off" => {
                            let inner = Vec::<MouseOff>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::MouseOff(inner));
                        }
                        "mode_change" => {
                            let inner = Vec::<ModeChange>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::ModeChange(inner));
                        }
                        "bell" => {
                            let inner = Vec::<Bell>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::Bell(inner));
                        }
                        "visual_bell" => {
                            let inner = Vec::<VisualBell>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::VisualBell(inner));
                        }
                        "flush" => {
                            let inner = Vec::<Flush>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::Flush(inner));
                        }
                        "suspend" => {
                            let inner = Vec::<Suspend>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::Suspend(inner));
                        }
                        "set_title" => {
                            let inner = Vec::<SetTitle>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::SetTitle(inner));
                        }
                        "set_icon" => {
                            let inner = Vec::<SetIcon>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::SetIcon(inner));
                        }
                        "screenshot" => {
                            let inner = Vec::<Screenshot>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::Screenshot(inner));
                        }
                        "option_set" => {
                            let inner = Vec::<OptionSet>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::OptionSet(inner));
                        }
                        "chdir" => {
                            let inner = Vec::<Chdir>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::Chdir(inner));
                        }
                        "update_fg" => {
                            let inner = Vec::<UpdateFg>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::UpdateFg(inner));
                        }
                        "update_bg" => {
                            let inner = Vec::<UpdateBg>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::UpdateBg(inner));
                        }
                        "update_sp" => {
                            let inner = Vec::<UpdateSp>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::UpdateSp(inner));
                        }
                        "resize" => {
                            let inner = Vec::<Resize>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::Resize(inner));
                        }
                        "clear" => {
                            let inner = Vec::<Clear>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::Clear(inner));
                        }
                        "eol_clear" => {
                            let inner = Vec::<EolClear>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::EolClear(inner));
                        }
                        "cursor_goto" => {
                            let inner = Vec::<CursorGoto>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::CursorGoto(inner));
                        }
                        "highlight_set" => {
                            let inner = Vec::<HighlightSet>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::HighlightSet(inner));
                        }
                        "put" => {
                            let inner = Vec::<Put>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::Put(inner));
                        }
                        "set_scroll_region" => {
                            let inner = Vec::<SetScrollRegion>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::SetScrollRegion(inner));
                        }
                        "scroll" => {
                            let inner = Vec::<Scroll>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::Scroll(inner));
                        }
                        "default_colors_set" => {
                            let inner = Vec::<DefaultColorsSet>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::DefaultColorsSet(inner));
                        }
                        "hl_attr_define" => {
                            let inner = Vec::<HlAttrDefine>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::HlAttrDefine(inner));
                        }
                        "hl_group_set" => {
                            let inner = Vec::<HlGroupSet>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::HlGroupSet(inner));
                        }
                        "grid_resize" => {
                            let inner = Vec::<GridResize>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::GridResize(inner));
                        }
                        "grid_clear" => {
                            let inner = Vec::<GridClear>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::GridClear(inner));
                        }
                        "grid_cursor_goto" => {
                            let inner = Vec::<GridCursorGoto>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::GridCursorGoto(inner));
                        }
                        "grid_line" => {
                            let inner = Vec::<GridLine>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::GridLine(inner));
                        }
                        "grid_scroll" => {
                            let inner = Vec::<GridScroll>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::GridScroll(inner));
                        }
                        "grid_destroy" => {
                            let inner = Vec::<GridDestroy>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::GridDestroy(inner));
                        }
                        "win_pos" => {
                            let inner = Vec::<WinPos>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::WinPos(inner));
                        }
                        "win_float_pos" => {
                            let inner = Vec::<WinFloatPos>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::WinFloatPos(inner));
                        }
                        "win_external_pos" => {
                            let inner = Vec::<WinExternalPos>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::WinExternalPos(inner));
                        }
                        "win_hide" => {
                            let inner = Vec::<WinHide>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::WinHide(inner));
                        }
                        "win_close" => {
                            let inner = Vec::<WinClose>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::WinClose(inner));
                        }
                        "msg_set_pos" => {
                            let inner = Vec::<MsgSetPos>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::MsgSetPos(inner));
                        }
                        "win_viewport" => {
                            let inner = Vec::<WinViewport>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::WinViewport(inner));
                        }
                        "win_viewport_margins" => {
                            let inner = Vec::<WinViewportMargins>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::WinViewportMargins(inner));
                        }
                        "win_extmark" => {
                            let inner = Vec::<WinExtmark>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::WinExtmark(inner));
                        }
                        "popupmenu_show" => {
                            let inner = Vec::<PopupmenuShow>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::PopupmenuShow(inner));
                        }
                        "popupmenu_hide" => {
                            let inner = Vec::<PopupmenuHide>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::PopupmenuHide(inner));
                        }
                        "popupmenu_select" => {
                            let inner = Vec::<PopupmenuSelect>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::PopupmenuSelect(inner));
                        }
                        "tabline_update" => {
                            let inner = Vec::<TablineUpdate>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::TablineUpdate(inner));
                        }
                        "cmdline_show" => {
                            let inner = Vec::<CmdlineShow>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::CmdlineShow(inner));
                        }
                        "cmdline_pos" => {
                            let inner = Vec::<CmdlinePos>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::CmdlinePos(inner));
                        }
                        "cmdline_special_char" => {
                            let inner = Vec::<CmdlineSpecialChar>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::CmdlineSpecialChar(inner));
                        }
                        "cmdline_hide" => {
                            let inner = Vec::<CmdlineHide>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::CmdlineHide(inner));
                        }
                        "cmdline_block_show" => {
                            let inner = Vec::<CmdlineBlockShow>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::CmdlineBlockShow(inner));
                        }
                        "cmdline_block_append" => {
                            let inner = Vec::<CmdlineBlockAppend>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::CmdlineBlockAppend(inner));
                        }
                        "cmdline_block_hide" => {
                            let inner = Vec::<CmdlineBlockHide>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::CmdlineBlockHide(inner));
                        }
                        "wildmenu_show" => {
                            let inner = Vec::<WildmenuShow>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::WildmenuShow(inner));
                        }
                        "wildmenu_select" => {
                            let inner = Vec::<WildmenuSelect>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::WildmenuSelect(inner));
                        }
                        "wildmenu_hide" => {
                            let inner = Vec::<WildmenuHide>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::WildmenuHide(inner));
                        }
                        "msg_show" => {
                            let inner = Vec::<MsgShow>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::MsgShow(inner));
                        }
                        "msg_clear" => {
                            let inner = Vec::<MsgClear>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::MsgClear(inner));
                        }
                        "msg_showcmd" => {
                            let inner = Vec::<MsgShowcmd>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::MsgShowcmd(inner));
                        }
                        "msg_showmode" => {
                            let inner = Vec::<MsgShowmode>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::MsgShowmode(inner));
                        }
                        "msg_ruler" => {
                            let inner = Vec::<MsgRuler>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::MsgRuler(inner));
                        }
                        "msg_history_show" => {
                            let inner = Vec::<MsgHistoryShow>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::MsgHistoryShow(inner));
                        }
                        "msg_history_clear" => {
                            let inner = Vec::<MsgHistoryClear>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::MsgHistoryClear(inner));
                        }
                        "error_exit" => {
                            let inner = Vec::<ErrorExit>::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::ErrorExit(inner));
                        }

                        o => {
                            let inner = Value::deserialize(ContSeq::new(seq))?;
                            return Ok(UiEvent::Unknown(Box::new((o.to_string(), inner))));
                        }
                    }
                }
            }
        }
    }
}
pub use uievent::UiEvent;
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
