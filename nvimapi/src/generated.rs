
use crate::nvimapi::{TABPAGE_ID, WINDOW_ID, BUFFER_ID};
use log::debug;
use serde::Deserializer;
use crate::contseq::ContSeq;
use crate::TryFromValue;
use rmpv::Value;
use crate::Pairs;
use crate::error;
use serde::{Deserialize, Serialize};
type Boolean = bool;
type Integer = i64;
type Float = f64  ;
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Buffer(pub Value);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Window(pub Value);
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Tabpage(pub Value);
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
        fn send_response_wv(&self, msgid: i32, error: Value, result: Value) -> error::Result<()>;
        fn send_response(&self, msgid: i32, error: impl serde::Serialize, result: impl serde::Serialize) -> error::Result<()>;
        async fn call_fn_wv<R>(&self, fn_name: String, args: impl crate::valueseq::ValueSeq) -> error::Result<R>
        where 
            R: TryFromValue;
        async fn call_fn<D,S>(&self, fn_name: &str, args: S) -> error::Result<D>
        where 
            D: Deserialize<'static>,
            S: crate::valueseq::SerialSeq;
    ///nvim_get_autocmds({opts})                                *nvim_get_autocmds()*
///    Get all autocommands that match the corresponding {opts}.
///
///    These examples will get autocommands matching ALL the given criteria: >lua
///        -- Matches all criteria
///        autocommands = vim.api.nvim_get_autocmds({
///          group = 'MyGroup',
///          event = {'BufEnter', 'BufWinEnter'},
///          pattern = {'*.c', '*.h'}
///        })
///
///        -- All commands from one group
///        autocommands = vim.api.nvim_get_autocmds({
///          group = 'MyGroup',
///        })
///<
///
///    NOTE: When multiple patterns or events are provided, it will find all the
///    autocommands that match any combination of them.
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {opts}  Dict with at least one of the following:
///                • buffer: (integer) Buffer number or list of buffer numbers
///                  for buffer local autocommands |autocmd-buflocal|. Cannot be
///                  used with {pattern}
///                • event: (string|table) event or events to match against
///                  |autocmd-events|.
///                • id: (integer) Autocommand ID to match.
///                • group: (string|table) the autocommand group name or id to
///                  match against.
///                • pattern: (string|table) pattern or patterns to match against
///                  |autocmd-pattern|. Cannot be used with {buffer}
///
///    Return: ~
///        Array of autocommands matching the criteria, with each item containing
///        the following fields:
///        • buffer: (integer) the buffer number.
///        • buflocal: (boolean) true if the autocommand is buffer local.
///        • command: (string) the autocommand command. Note: this will be empty
///          if a callback is set.
///        • callback: (function|string|nil): Lua function or name of a Vim
///          script function which is executed when this autocommand is
///          triggered.
///        • desc: (string) the autocommand description.
///        • event: (string) the autocommand event.
///        • id: (integer) the autocommand id (only when defined with the API).
///        • group: (integer) the autocommand group id.
///        • group_name: (string) the autocommand group name.
///        • once: (boolean) whether the autocommand is only run once.
///        • pattern: (string) the autocommand pattern. If the autocommand is
///          buffer local |autocmd-buffer-local|:
///
///
///==============================================================================
async fn get_autocmds<D: Deserialize<'static>>(&self, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_get_autocmds".into(), (opts, )).await
}
///nvim_get_autocmds({opts})                                *nvim_get_autocmds()*
///    Get all autocommands that match the corresponding {opts}.
///
///    These examples will get autocommands matching ALL the given criteria: >lua
///        -- Matches all criteria
///        autocommands = vim.api.nvim_get_autocmds({
///          group = 'MyGroup',
///          event = {'BufEnter', 'BufWinEnter'},
///          pattern = {'*.c', '*.h'}
///        })
///
///        -- All commands from one group
///        autocommands = vim.api.nvim_get_autocmds({
///          group = 'MyGroup',
///        })
///<
///
///    NOTE: When multiple patterns or events are provided, it will find all the
///    autocommands that match any combination of them.
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {opts}  Dict with at least one of the following:
///                • buffer: (integer) Buffer number or list of buffer numbers
///                  for buffer local autocommands |autocmd-buflocal|. Cannot be
///                  used with {pattern}
///                • event: (string|table) event or events to match against
///                  |autocmd-events|.
///                • id: (integer) Autocommand ID to match.
///                • group: (string|table) the autocommand group name or id to
///                  match against.
///                • pattern: (string|table) pattern or patterns to match against
///                  |autocmd-pattern|. Cannot be used with {buffer}
///
///    Return: ~
///        Array of autocommands matching the criteria, with each item containing
///        the following fields:
///        • buffer: (integer) the buffer number.
///        • buflocal: (boolean) true if the autocommand is buffer local.
///        • command: (string) the autocommand command. Note: this will be empty
///          if a callback is set.
///        • callback: (function|string|nil): Lua function or name of a Vim
///          script function which is executed when this autocommand is
///          triggered.
///        • desc: (string) the autocommand description.
///        • event: (string) the autocommand event.
///        • id: (integer) the autocommand id (only when defined with the API).
///        • group: (integer) the autocommand group id.
///        • group_name: (string) the autocommand group name.
///        • once: (boolean) whether the autocommand is only run once.
///        • pattern: (string) the autocommand pattern. If the autocommand is
///          buffer local |autocmd-buffer-local|:
///
///
///==============================================================================
async fn get_autocmds_wv(&self, opts: Dict, ) -> error::Result<Array> {
	self.call_fn_wv("nvim_get_autocmds".into(), (opts, )).await
}
///nvim_create_autocmd({event}, {opts})                   *nvim_create_autocmd()*
///    Creates an |autocommand| event handler, defined by `callback` (Lua
///    function or Vimscript function name string) or `command` (Ex command
///    string).
///
///    Example using Lua callback: >lua
///        vim.api.nvim_create_autocmd({'BufEnter', 'BufWinEnter'}, {
///          pattern = {'*.c', '*.h'},
///          callback = function(ev)
///            print(string.format('event fired: %s', vim.inspect(ev)))
///          end
///        })
///<
///
///    Example using an Ex command as the handler: >lua
///        vim.api.nvim_create_autocmd({'BufEnter', 'BufWinEnter'}, {
///          pattern = {'*.c', '*.h'},
///          command = "echo 'Entering a C or C++ file'",
///        })
///<
///
///    Note: `pattern` is NOT automatically expanded (unlike with |:autocmd|),
///    thus names like "$HOME" and "~" must be expanded explicitly: >lua
///        pattern = vim.fn.expand('~') .. '/some/path/*.py'
///<
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {event}  (string|array) Event(s) that will trigger the handler
///                 (`callback` or `command`).
///      • {opts}   Options dict:
///                 • group (string|integer) optional: autocommand group name or
///                   id to match against.
///                 • pattern (string|array) optional: pattern(s) to match
///                   literally |autocmd-pattern|.
///                 • buffer (integer) optional: buffer number for buffer-local
///                   autocommands |autocmd-buflocal|. Cannot be used with
///                   {pattern}.
///                 • desc (string) optional: description (for documentation and
///                   troubleshooting).
///                 • callback (function|string) optional: Lua function (or
///                   Vimscript function name, if string) called when the
///                   event(s) is triggered. Lua callback can return a truthy
///                   value (not `false` or `nil`) to delete the autocommand, and
///                   receives one argument, a table with these keys:
async fn create_autocmd(&self, event: impl Serialize, opts: impl Serialize, ) -> error::Result<Integer> {
	self.call_fn("nvim_create_autocmd".into(), (event, opts, )).await
}
///nvim_create_autocmd({event}, {opts})                   *nvim_create_autocmd()*
///    Creates an |autocommand| event handler, defined by `callback` (Lua
///    function or Vimscript function name string) or `command` (Ex command
///    string).
///
///    Example using Lua callback: >lua
///        vim.api.nvim_create_autocmd({'BufEnter', 'BufWinEnter'}, {
///          pattern = {'*.c', '*.h'},
///          callback = function(ev)
///            print(string.format('event fired: %s', vim.inspect(ev)))
///          end
///        })
///<
///
///    Example using an Ex command as the handler: >lua
///        vim.api.nvim_create_autocmd({'BufEnter', 'BufWinEnter'}, {
///          pattern = {'*.c', '*.h'},
///          command = "echo 'Entering a C or C++ file'",
///        })
///<
///
///    Note: `pattern` is NOT automatically expanded (unlike with |:autocmd|),
///    thus names like "$HOME" and "~" must be expanded explicitly: >lua
///        pattern = vim.fn.expand('~') .. '/some/path/*.py'
///<
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {event}  (string|array) Event(s) that will trigger the handler
///                 (`callback` or `command`).
///      • {opts}   Options dict:
///                 • group (string|integer) optional: autocommand group name or
///                   id to match against.
///                 • pattern (string|array) optional: pattern(s) to match
///                   literally |autocmd-pattern|.
///                 • buffer (integer) optional: buffer number for buffer-local
///                   autocommands |autocmd-buflocal|. Cannot be used with
///                   {pattern}.
///                 • desc (string) optional: description (for documentation and
///                   troubleshooting).
///                 • callback (function|string) optional: Lua function (or
///                   Vimscript function name, if string) called when the
///                   event(s) is triggered. Lua callback can return a truthy
///                   value (not `false` or `nil`) to delete the autocommand, and
///                   receives one argument, a table with these keys:
async fn create_autocmd_wv(&self, event: Object, opts: Dict, ) -> error::Result<Integer> {
	self.call_fn_wv("nvim_create_autocmd".into(), (event, opts, )).await
}
///nvim_del_autocmd({id})                                    *nvim_del_autocmd()*
///    Deletes an autocommand by id.
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {id}  Integer Autocommand id returned by |nvim_create_autocmd()|
///
async fn del_autocmd(&self, id: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_del_autocmd".into(), (id, )).await
}
///nvim_clear_autocmds({opts})                            *nvim_clear_autocmds()*
///    Clears all autocommands selected by {opts}. To delete autocmds see
///    |nvim_del_autocmd()|.
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {opts}  Parameters
///                • event: (string|table) Examples:
///                  • event: "pat1"
///                  • event: { "pat1" }
///                  • event: { "pat1", "pat2", "pat3" }
///                • pattern: (string|table)
///                  • pattern or patterns to match exactly.
///                    • For example, if you have `*.py` as that pattern for the
///                      autocmd, you must pass `*.py` exactly to clear it.
///                      `test.py` will not match the pattern.
///                  • defaults to clearing all patterns.
///                  • NOTE: Cannot be used with {buffer}
///                • buffer: (bufnr)
///                  • clear only |autocmd-buflocal| autocommands.
///                  • NOTE: Cannot be used with {pattern}
///                • group: (string|int) The augroup name or id.
///                  • NOTE: If not passed, will only delete autocmds not in any
///                    group.
///
async fn clear_autocmds(&self, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_clear_autocmds".into(), (opts, )).await
}
///nvim_clear_autocmds({opts})                            *nvim_clear_autocmds()*
///    Clears all autocommands selected by {opts}. To delete autocmds see
///    |nvim_del_autocmd()|.
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {opts}  Parameters
///                • event: (string|table) Examples:
///                  • event: "pat1"
///                  • event: { "pat1" }
///                  • event: { "pat1", "pat2", "pat3" }
///                • pattern: (string|table)
///                  • pattern or patterns to match exactly.
///                    • For example, if you have `*.py` as that pattern for the
///                      autocmd, you must pass `*.py` exactly to clear it.
///                      `test.py` will not match the pattern.
///                  • defaults to clearing all patterns.
///                  • NOTE: Cannot be used with {buffer}
///                • buffer: (bufnr)
///                  • clear only |autocmd-buflocal| autocommands.
///                  • NOTE: Cannot be used with {pattern}
///                • group: (string|int) The augroup name or id.
///                  • NOTE: If not passed, will only delete autocmds not in any
///                    group.
///
async fn clear_autocmds_wv(&self, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_clear_autocmds".into(), (opts, )).await
}
///nvim_create_augroup({name}, {opts})                    *nvim_create_augroup()*
///    Create or get an autocommand group |autocmd-groups|.
///
///    To get an existing group id, do: >lua
///        local id = vim.api.nvim_create_augroup('my.lsp.config', {
///            clear = false
///        })
///<
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {name}  String: The name of the group
///      • {opts}  Dict Parameters
///                • clear (bool) optional: defaults to true. Clear existing
///                  commands if the group already exists |autocmd-groups|.
///
///    Return: ~
///        Integer id of the created group.
///
///    See also: ~
///      • |autocmd-groups|
///
async fn create_augroup(&self, name: &str, opts: impl Serialize, ) -> error::Result<Integer> {
	self.call_fn("nvim_create_augroup".into(), (name, opts, )).await
}
///nvim_create_augroup({name}, {opts})                    *nvim_create_augroup()*
///    Create or get an autocommand group |autocmd-groups|.
///
///    To get an existing group id, do: >lua
///        local id = vim.api.nvim_create_augroup('my.lsp.config', {
///            clear = false
///        })
///<
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {name}  String: The name of the group
///      • {opts}  Dict Parameters
///                • clear (bool) optional: defaults to true. Clear existing
///                  commands if the group already exists |autocmd-groups|.
///
///    Return: ~
///        Integer id of the created group.
///
///    See also: ~
///      • |autocmd-groups|
///
async fn create_augroup_wv(&self, name: String, opts: Dict, ) -> error::Result<Integer> {
	self.call_fn_wv("nvim_create_augroup".into(), (name, opts, )).await
}
///nvim_del_augroup_by_id({id})                        *nvim_del_augroup_by_id()*
///    Delete an autocommand group by id.
///
///    To get a group id one can use |nvim_get_autocmds()|.
///
///    NOTE: behavior differs from |:augroup-delete|. When deleting a group,
///    autocommands contained in this group will also be deleted and cleared.
///    This group will no longer exist.
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {id}  Integer The id of the group.
///
///    See also: ~
///      • |nvim_del_augroup_by_name()|
///      • |nvim_create_augroup()|
///
async fn del_augroup_by_id(&self, id: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_del_augroup_by_id".into(), (id, )).await
}
///nvim_del_augroup_by_name({name})                  *nvim_del_augroup_by_name()*
///    Delete an autocommand group by name.
///
///    NOTE: behavior differs from |:augroup-delete|. When deleting a group,
///    autocommands contained in this group will also be deleted and cleared.
///    This group will no longer exist.
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {name}  String The name of the group.
///
///    See also: ~
///      • |autocmd-groups|
///
async fn del_augroup_by_name(&self, name: &str, ) -> error::Result<()> {
	self.call_fn("nvim_del_augroup_by_name".into(), (name, )).await
}
///nvim_exec_autocmds({event}, {opts})                     *nvim_exec_autocmds()*
///    Execute all autocommands for {event} that match the corresponding {opts}
///    |autocmd-execute|.
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {event}  (String|Array) The event or events to execute
///      • {opts}   Dict of autocommand options:
///                 • group (string|integer) optional: the autocommand group name
///                   or id to match against. |autocmd-groups|.
///                 • pattern (string|array) optional: defaults to "*"
///                   |autocmd-pattern|. Cannot be used with {buffer}.
///                 • buffer (integer) optional: buffer number
///                   |autocmd-buflocal|. Cannot be used with {pattern}.
///                 • modeline (bool) optional: defaults to true. Process the
///                   modeline after the autocommands <nomodeline>.
///                 • data (any): arbitrary data to send to the autocommand
///                   callback. See |nvim_create_autocmd()| for details.
///
///    See also: ~
///      • |:doautocmd|
///
async fn exec_autocmds(&self, event: impl Serialize, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_exec_autocmds".into(), (event, opts, )).await
}
///nvim_exec_autocmds({event}, {opts})                     *nvim_exec_autocmds()*
///    Execute all autocommands for {event} that match the corresponding {opts}
///    |autocmd-execute|.
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {event}  (String|Array) The event or events to execute
///      • {opts}   Dict of autocommand options:
///                 • group (string|integer) optional: the autocommand group name
///                   or id to match against. |autocmd-groups|.
///                 • pattern (string|array) optional: defaults to "*"
///                   |autocmd-pattern|. Cannot be used with {buffer}.
///                 • buffer (integer) optional: buffer number
///                   |autocmd-buflocal|. Cannot be used with {pattern}.
///                 • modeline (bool) optional: defaults to true. Process the
///                   modeline after the autocommands <nomodeline>.
///                 • data (any): arbitrary data to send to the autocommand
///                   callback. See |nvim_create_autocmd()| for details.
///
///    See also: ~
///      • |:doautocmd|
///
async fn exec_autocmds_wv(&self, event: Object, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_exec_autocmds".into(), (event, opts, )).await
}
///nvim_buf_line_count({buffer})                          *nvim_buf_line_count()*
///    Returns the number of lines in the given buffer.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///
///    Return: ~
///        Line count, or 0 for unloaded buffer. |api-buffer|
///
async fn buf_line_count(&self, buffer: Buffer, ) -> error::Result<Integer> {
	self.call_fn("nvim_buf_line_count".into(), (buffer, )).await
}
///nvim_buf_attach({buffer}, {send_buffer}, {opts})           *nvim_buf_attach()*
///    Activates buffer-update events on a channel, or as Lua callbacks.
///
///    Example (Lua): capture buffer updates in a global `events` variable (use
///    "vim.print(events)" to see its contents): >lua
///        events = {}
///        vim.api.nvim_buf_attach(0, false, {
///          on_lines = function(...)
///            table.insert(events, {...})
///          end,
///        })
///<
///
///    Attributes: ~
///        Since: 0.3.0
///
///    Parameters: ~
///      • {buffer}       Buffer id, or 0 for current buffer
///      • {send_buffer}  True if the initial notification should contain the
///                       whole buffer: first notification will be
///                       `nvim_buf_lines_event`. Else the first notification
///                       will be `nvim_buf_changedtick_event`. Not for Lua
///                       callbacks.
///      • {opts}         Optional parameters.
///                       • on_lines: Lua callback invoked on change. Return a
///                         truthy value (not `false` or `nil`) to detach. Args:
///                         • the string "lines"
///                         • buffer id
///                         • b:changedtick
///                         • first line that changed (zero-indexed)
///                         • last line that was changed
///                         • last line in the updated range
///                         • byte count of previous contents
///                         • deleted_codepoints (if `utf_sizes` is true)
///                         • deleted_codeunits (if `utf_sizes` is true)
///                       • on_bytes: Lua callback invoked on change. This
///                         callback receives more granular information about the
///                         change compared to on_lines. Return a truthy value
///                         (not `false` or `nil`) to detach. Args:
///                         • the string "bytes"
///                         • buffer id
///                         • b:changedtick
///                         • start row of the changed text (zero-indexed)
///                         • start column of the changed text
///                         • byte offset of the changed text (from the start of
///                           the buffer)
///                         • old end row of the changed text (offset from start
///                           row)
///                         • old end column of the changed text (if old end row
///                           = 0, offset from start column)
///                         • old end byte length of the changed text
///                         • new end row of the changed text (offset from start
///                           row)
///                         • new end column of the changed text (if new end row
///                           = 0, offset from start column)
///                         • new end byte length of the changed text
///                       • on_changedtick: Lua callback invoked on changedtick
///                         increment without text change. Args:
///                         • the string "changedtick"
///                         • buffer id
///                         • b:changedtick
///                       • on_detach: Lua callback invoked on detach. Args:
///                         • the string "detach"
///                         • buffer id
///                       • on_reload: Lua callback invoked on reload. The entire
///                         buffer content should be considered changed. Args:
///                         • the string "reload"
///                         • buffer id
///                       • utf_sizes: include UTF-32 and UTF-16 size of the
///                         replaced region, as args to `on_lines`.
///                       • preview: also attach to command preview (i.e.
///                         'inccommand') events.
///
///    Return: ~
///        False if attach failed (invalid parameter, or buffer isn't loaded);
///        otherwise True. TODO: LUA_API_NO_EVAL
///
///    See also: ~
///      • |nvim_buf_detach()|
///      • |api-buffer-updates-lua|
///
async fn buf_attach(&self, buffer: Buffer, send_buffer: Boolean, opts: impl Serialize, ) -> error::Result<Boolean> {
	self.call_fn("nvim_buf_attach".into(), (buffer, send_buffer, opts, )).await
}
///nvim_buf_attach({buffer}, {send_buffer}, {opts})           *nvim_buf_attach()*
///    Activates buffer-update events on a channel, or as Lua callbacks.
///
///    Example (Lua): capture buffer updates in a global `events` variable (use
///    "vim.print(events)" to see its contents): >lua
///        events = {}
///        vim.api.nvim_buf_attach(0, false, {
///          on_lines = function(...)
///            table.insert(events, {...})
///          end,
///        })
///<
///
///    Attributes: ~
///        Since: 0.3.0
///
///    Parameters: ~
///      • {buffer}       Buffer id, or 0 for current buffer
///      • {send_buffer}  True if the initial notification should contain the
///                       whole buffer: first notification will be
///                       `nvim_buf_lines_event`. Else the first notification
///                       will be `nvim_buf_changedtick_event`. Not for Lua
///                       callbacks.
///      • {opts}         Optional parameters.
///                       • on_lines: Lua callback invoked on change. Return a
///                         truthy value (not `false` or `nil`) to detach. Args:
///                         • the string "lines"
///                         • buffer id
///                         • b:changedtick
///                         • first line that changed (zero-indexed)
///                         • last line that was changed
///                         • last line in the updated range
///                         • byte count of previous contents
///                         • deleted_codepoints (if `utf_sizes` is true)
///                         • deleted_codeunits (if `utf_sizes` is true)
///                       • on_bytes: Lua callback invoked on change. This
///                         callback receives more granular information about the
///                         change compared to on_lines. Return a truthy value
///                         (not `false` or `nil`) to detach. Args:
///                         • the string "bytes"
///                         • buffer id
///                         • b:changedtick
///                         • start row of the changed text (zero-indexed)
///                         • start column of the changed text
///                         • byte offset of the changed text (from the start of
///                           the buffer)
///                         • old end row of the changed text (offset from start
///                           row)
///                         • old end column of the changed text (if old end row
///                           = 0, offset from start column)
///                         • old end byte length of the changed text
///                         • new end row of the changed text (offset from start
///                           row)
///                         • new end column of the changed text (if new end row
///                           = 0, offset from start column)
///                         • new end byte length of the changed text
///                       • on_changedtick: Lua callback invoked on changedtick
///                         increment without text change. Args:
///                         • the string "changedtick"
///                         • buffer id
///                         • b:changedtick
///                       • on_detach: Lua callback invoked on detach. Args:
///                         • the string "detach"
///                         • buffer id
///                       • on_reload: Lua callback invoked on reload. The entire
///                         buffer content should be considered changed. Args:
///                         • the string "reload"
///                         • buffer id
///                       • utf_sizes: include UTF-32 and UTF-16 size of the
///                         replaced region, as args to `on_lines`.
///                       • preview: also attach to command preview (i.e.
///                         'inccommand') events.
///
///    Return: ~
///        False if attach failed (invalid parameter, or buffer isn't loaded);
///        otherwise True. TODO: LUA_API_NO_EVAL
///
///    See also: ~
///      • |nvim_buf_detach()|
///      • |api-buffer-updates-lua|
///
async fn buf_attach_wv(&self, buffer: Buffer, send_buffer: Boolean, opts: Dict, ) -> error::Result<Boolean> {
	self.call_fn_wv("nvim_buf_attach".into(), (buffer, send_buffer, opts, )).await
}
///nvim_buf_detach({buffer})                                  *nvim_buf_detach()*
///    Deactivates buffer-update events on the channel.
///
///    Attributes: ~
///        |RPC| only
///        Since: 0.3.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///
///    Return: ~
///        False if detach failed (because the buffer isn't loaded); otherwise
///        True.
///
///    See also: ~
///      • |nvim_buf_attach()|
///      • |api-lua-detach| for detaching Lua callbacks
///
async fn buf_detach(&self, buffer: Buffer, ) -> error::Result<Boolean> {
	self.call_fn("nvim_buf_detach".into(), (buffer, )).await
}
///                                                        *nvim_buf_get_lines()*
///nvim_buf_get_lines({buffer}, {start}, {end}, {strict_indexing})
///    Gets a line-range from the buffer.
///
///    Indexing is zero-based, end-exclusive. Negative indices are interpreted as
///    length+1+index: -1 refers to the index past the end. So to get the last
///    element use start=-2 and end=-1.
///
///    Out-of-bounds indices are clamped to the nearest valid value, unless
///    `strict_indexing` is set.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {buffer}           Buffer id, or 0 for current buffer
///      • {start}            First line index
///      • {end}              Last line index, exclusive
///      • {strict_indexing}  Whether out-of-bounds should be an error.
///
///    Return: ~
///        Array of lines, or empty array for unloaded buffer.
///
///    See also: ~
///      • |nvim_buf_get_text()|
///
async fn buf_get_lines(&self, buffer: Buffer, start: Integer, end: Integer, strict_indexing: Boolean, ) -> error::Result<Vec<String>> {
	self.call_fn("nvim_buf_get_lines".into(), (buffer, start, end, strict_indexing, )).await
}
///                                                        *nvim_buf_set_lines()*
///nvim_buf_set_lines({buffer}, {start}, {end}, {strict_indexing}, {replacement})
///    Sets (replaces) a line-range in the buffer.
///
///    Indexing is zero-based, end-exclusive. Negative indices are interpreted as
///    length+1+index: -1 refers to the index past the end. So to change or
///    delete the last line use start=-2 and end=-1.
///
///    To insert lines at a given index, set `start` and `end` to the same index.
///    To delete a range of lines, set `replacement` to an empty array.
///
///    Out-of-bounds indices are clamped to the nearest valid value, unless
///    `strict_indexing` is set.
///
///    Attributes: ~
///        not allowed when |textlock| is active
///        Since: 0.1.0
///
///    Parameters: ~
///      • {buffer}           Buffer id, or 0 for current buffer
///      • {start}            First line index
///      • {end}              Last line index, exclusive
///      • {strict_indexing}  Whether out-of-bounds should be an error.
///      • {replacement}      Array of lines to use as replacement
///
///    See also: ~
///      • |nvim_buf_set_text()|
///
async fn buf_set_lines(&self, buffer: Buffer, start: Integer, end: Integer, strict_indexing: Boolean, replacement: &[&str], ) -> error::Result<()> {
	self.call_fn("nvim_buf_set_lines".into(), (buffer, start, end, strict_indexing, replacement, )).await
}
///                                                         *nvim_buf_set_text()*
///nvim_buf_set_text({buffer}, {start_row}, {start_col}, {end_row}, {end_col},
///                  {replacement})
///    Sets (replaces) a range in the buffer
///
///    This is recommended over |nvim_buf_set_lines()| when only modifying parts
///    of a line, as extmarks will be preserved on non-modified parts of the
///    touched lines.
///
///    Indexing is zero-based. Row indices are end-inclusive, and column indices
///    are end-exclusive.
///
///    To insert text at a given `(row, column)` location, use
///    `start_row = end_row = row` and `start_col = end_col = col`. To delete the
///    text in a range, use `replacement = {}`.
///
///    Note: ~
///      • Prefer |nvim_buf_set_lines()| (for performance) to add or delete
///        entire lines.
///      • Prefer |nvim_paste()| or |nvim_put()| to insert (instead of replace)
///        text at cursor.
///
///    Attributes: ~
///        not allowed when |textlock| is active
///        Since: 0.5.0
///
///    Parameters: ~
///      • {buffer}       Buffer id, or 0 for current buffer
///      • {start_row}    First line index
///      • {start_col}    Starting column (byte offset) on first line
///      • {end_row}      Last line index, inclusive
///      • {end_col}      Ending column (byte offset) on last line, exclusive
///      • {replacement}  Array of lines to use as replacement
///
async fn buf_set_text(&self, buffer: Buffer, start_row: Integer, start_col: Integer, end_row: Integer, end_col: Integer, replacement: &[&str], ) -> error::Result<()> {
	self.call_fn("nvim_buf_set_text".into(), (buffer, start_row, start_col, end_row, end_col, replacement, )).await
}
///                                                         *nvim_buf_get_text()*
///nvim_buf_get_text({buffer}, {start_row}, {start_col}, {end_row}, {end_col},
///                  {opts})
///    Gets a range from the buffer (may be partial lines, unlike
///    |nvim_buf_get_lines()|).
///
///    Indexing is zero-based. Row indices are end-inclusive, and column indices
///    are end-exclusive.
///
///    Prefer |nvim_buf_get_lines()| when retrieving entire lines.
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {buffer}     Buffer id, or 0 for current buffer
///      • {start_row}  First line index
///      • {start_col}  Starting column (byte offset) on first line
///      • {end_row}    Last line index, inclusive
///      • {end_col}    Ending column (byte offset) on last line, exclusive
///      • {opts}       Optional parameters. Currently unused.
///
///    Return: ~
///        Array of lines, or empty array for unloaded buffer.
///
async fn buf_get_text(&self, buffer: Buffer, start_row: Integer, start_col: Integer, end_row: Integer, end_col: Integer, opts: impl Serialize, ) -> error::Result<Vec<String>> {
	self.call_fn("nvim_buf_get_text".into(), (buffer, start_row, start_col, end_row, end_col, opts, )).await
}
///                                                         *nvim_buf_get_text()*
///nvim_buf_get_text({buffer}, {start_row}, {start_col}, {end_row}, {end_col},
///                  {opts})
///    Gets a range from the buffer (may be partial lines, unlike
///    |nvim_buf_get_lines()|).
///
///    Indexing is zero-based. Row indices are end-inclusive, and column indices
///    are end-exclusive.
///
///    Prefer |nvim_buf_get_lines()| when retrieving entire lines.
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {buffer}     Buffer id, or 0 for current buffer
///      • {start_row}  First line index
///      • {start_col}  Starting column (byte offset) on first line
///      • {end_row}    Last line index, inclusive
///      • {end_col}    Ending column (byte offset) on last line, exclusive
///      • {opts}       Optional parameters. Currently unused.
///
///    Return: ~
///        Array of lines, or empty array for unloaded buffer.
///
async fn buf_get_text_wv(&self, buffer: Buffer, start_row: Integer, start_col: Integer, end_row: Integer, end_col: Integer, opts: Dict, ) -> error::Result<Vec<String>> {
	self.call_fn_wv("nvim_buf_get_text".into(), (buffer, start_row, start_col, end_row, end_col, opts, )).await
}
///nvim_buf_get_offset({buffer}, {index})                 *nvim_buf_get_offset()*
///    Returns the byte offset of a line (0-indexed). |api-indexing|
///
///    Line 1 (index=0) has offset 0. UTF-8 bytes are counted. EOL is one byte.
///    'fileformat' and 'fileencoding' are ignored. The line index just after the
///    last line gives the total byte-count of the buffer. A final EOL byte is
///    counted if it would be written, see 'eol'.
///
///    Unlike |line2byte()|, throws error for out-of-bounds indexing. Returns -1
///    for unloaded buffer.
///
///    Attributes: ~
///        Since: 0.3.2
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {index}   Line index
///
///    Return: ~
///        Integer byte offset, or -1 for unloaded buffer.
///
async fn buf_get_offset(&self, buffer: Buffer, index: Integer, ) -> error::Result<Integer> {
	self.call_fn("nvim_buf_get_offset".into(), (buffer, index, )).await
}
///nvim_buf_get_var({buffer}, {name})                        *nvim_buf_get_var()*
///    Gets a buffer-scoped (b:) variable.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {name}    Variable name
///
///    Return: ~
///        Variable value
///
async fn buf_get_var<D: Deserialize<'static>>(&self, buffer: Buffer, name: &str, ) -> error::Result<D> {
	self.call_fn("nvim_buf_get_var".into(), (buffer, name, )).await
}
///nvim_buf_get_var({buffer}, {name})                        *nvim_buf_get_var()*
///    Gets a buffer-scoped (b:) variable.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {name}    Variable name
///
///    Return: ~
///        Variable value
///
async fn buf_get_var_wv(&self, buffer: Buffer, name: String, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_buf_get_var".into(), (buffer, name, )).await
}
///nvim_buf_get_changedtick({buffer})                *nvim_buf_get_changedtick()*
///    Gets a changed tick of a buffer
///
///    Attributes: ~
///        Since: 0.2.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///
///    Return: ~
///        `b:changedtick` value.
///
async fn buf_get_changedtick(&self, buffer: Buffer, ) -> error::Result<Integer> {
	self.call_fn("nvim_buf_get_changedtick".into(), (buffer, )).await
}
///nvim_buf_get_keymap({buffer}, {mode})                  *nvim_buf_get_keymap()*
///    Gets a list of buffer-local |mapping| definitions.
///
///    Attributes: ~
///        Since: 0.2.1
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {mode}    Mode short-name ("n", "i", "v", ...)
///
///    Return: ~
///        Array of |maparg()|-like dictionaries describing mappings. The
///        "buffer" key holds the associated buffer id.
///
async fn buf_get_keymap<D: Deserialize<'static>>(&self, buffer: Buffer, mode: &str, ) -> error::Result<D> {
	self.call_fn("nvim_buf_get_keymap".into(), (buffer, mode, )).await
}
///nvim_buf_get_keymap({buffer}, {mode})                  *nvim_buf_get_keymap()*
///    Gets a list of buffer-local |mapping| definitions.
///
///    Attributes: ~
///        Since: 0.2.1
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {mode}    Mode short-name ("n", "i", "v", ...)
///
///    Return: ~
///        Array of |maparg()|-like dictionaries describing mappings. The
///        "buffer" key holds the associated buffer id.
///
async fn buf_get_keymap_wv(&self, buffer: Buffer, mode: String, ) -> error::Result<Vec<Dict>> {
	self.call_fn_wv("nvim_buf_get_keymap".into(), (buffer, mode, )).await
}
///                                                       *nvim_buf_set_keymap()*
///nvim_buf_set_keymap({buffer}, {mode}, {lhs}, {rhs}, {opts})
///    Sets a buffer-local |mapping| for the given mode.
///
///    Attributes: ~
///        Since: 0.4.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///
///    See also: ~
///      • |nvim_set_keymap()|
///
async fn buf_set_keymap(&self, buffer: Buffer, mode: &str, lhs: &str, rhs: &str, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_buf_set_keymap".into(), (buffer, mode, lhs, rhs, opts, )).await
}
///                                                       *nvim_buf_set_keymap()*
///nvim_buf_set_keymap({buffer}, {mode}, {lhs}, {rhs}, {opts})
///    Sets a buffer-local |mapping| for the given mode.
///
///    Attributes: ~
///        Since: 0.4.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///
///    See also: ~
///      • |nvim_set_keymap()|
///
async fn buf_set_keymap_wv(&self, buffer: Buffer, mode: String, lhs: String, rhs: String, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_buf_set_keymap".into(), (buffer, mode, lhs, rhs, opts, )).await
}
///nvim_buf_del_keymap({buffer}, {mode}, {lhs})           *nvim_buf_del_keymap()*
///    Unmaps a buffer-local |mapping| for the given mode.
///
///    Attributes: ~
///        Since: 0.4.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///
///    See also: ~
///      • |nvim_del_keymap()|
///
async fn buf_del_keymap(&self, buffer: Buffer, mode: &str, lhs: &str, ) -> error::Result<()> {
	self.call_fn("nvim_buf_del_keymap".into(), (buffer, mode, lhs, )).await
}
///nvim_buf_set_var({buffer}, {name}, {value})               *nvim_buf_set_var()*
///    Sets a buffer-scoped (b:) variable
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {name}    Variable name
///      • {value}   Variable value
///
///
///==============================================================================
async fn buf_set_var(&self, buffer: Buffer, name: &str, value: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_buf_set_var".into(), (buffer, name, value, )).await
}
///nvim_buf_set_var({buffer}, {name}, {value})               *nvim_buf_set_var()*
///    Sets a buffer-scoped (b:) variable
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {name}    Variable name
///      • {value}   Variable value
///
///
///==============================================================================
async fn buf_set_var_wv(&self, buffer: Buffer, name: String, value: Object, ) -> error::Result<()> {
	self.call_fn_wv("nvim_buf_set_var".into(), (buffer, name, value, )).await
}
///nvim_buf_del_var({buffer}, {name})                        *nvim_buf_del_var()*
///    Removes a buffer-scoped (b:) variable
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {name}    Variable name
///
async fn buf_del_var(&self, buffer: Buffer, name: &str, ) -> error::Result<()> {
	self.call_fn("nvim_buf_del_var".into(), (buffer, name, )).await
}
///nvim_buf_get_name({buffer})                              *nvim_buf_get_name()*
///    Gets the full file name for the buffer
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///
///    Return: ~
///        Buffer name
///
async fn buf_get_name(&self, buffer: Buffer, ) -> error::Result<String> {
	self.call_fn("nvim_buf_get_name".into(), (buffer, )).await
}
///nvim_buf_set_name({buffer}, {name})                      *nvim_buf_set_name()*
///    Sets the full file name for a buffer, like |:file_f|
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {name}    Buffer name
///
async fn buf_set_name(&self, buffer: Buffer, name: &str, ) -> error::Result<()> {
	self.call_fn("nvim_buf_set_name".into(), (buffer, name, )).await
}
///nvim_buf_is_loaded({buffer})                            *nvim_buf_is_loaded()*
///    Checks if a buffer is valid and loaded. See |api-buffer| for more info
///    about unloaded buffers.
///
///    Attributes: ~
///        Since: 0.3.2
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///
///    Return: ~
///        true if the buffer is valid and loaded, false otherwise.
///
async fn buf_is_loaded(&self, buffer: Buffer, ) -> error::Result<Boolean> {
	self.call_fn("nvim_buf_is_loaded".into(), (buffer, )).await
}
///nvim_buf_delete({buffer}, {opts})                          *nvim_buf_delete()*
///    Deletes the buffer. See |:bwipeout|
///
///    Attributes: ~
///        not allowed when |textlock| is active or in the |cmdwin|
///        Since: 0.5.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {opts}    Optional parameters. Keys:
///                  • force: Force deletion and ignore unsaved changes.
///                  • unload: Unloaded only, do not delete. See |:bunload|
///
async fn buf_delete(&self, buffer: Buffer, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_buf_delete".into(), (buffer, opts, )).await
}
///nvim_buf_delete({buffer}, {opts})                          *nvim_buf_delete()*
///    Deletes the buffer. See |:bwipeout|
///
///    Attributes: ~
///        not allowed when |textlock| is active or in the |cmdwin|
///        Since: 0.5.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {opts}    Optional parameters. Keys:
///                  • force: Force deletion and ignore unsaved changes.
///                  • unload: Unloaded only, do not delete. See |:bunload|
///
async fn buf_delete_wv(&self, buffer: Buffer, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_buf_delete".into(), (buffer, opts, )).await
}
///nvim_buf_is_valid({buffer})                              *nvim_buf_is_valid()*
///    Checks if a buffer is valid.
///
///    Note: ~
///      • Even if a buffer is valid it may have been unloaded. See |api-buffer|
///        for more info about unloaded buffers.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///
///    Return: ~
///        true if the buffer is valid, false otherwise.
///
async fn buf_is_valid(&self, buffer: Buffer, ) -> error::Result<Boolean> {
	self.call_fn("nvim_buf_is_valid".into(), (buffer, )).await
}
///nvim_buf_del_mark({buffer}, {name})                      *nvim_buf_del_mark()*
///    Deletes a named mark in the buffer. See |mark-motions|.
///
///    Note: ~
///      • only deletes marks set in the buffer, if the mark is not set in the
///        buffer it will return false.
///
///    Attributes: ~
///        Since: 0.6.0
///
///    Parameters: ~
///      • {buffer}  Buffer to set the mark on
///      • {name}    Mark name
///
///    Return: ~
///        true if the mark was deleted, else false.
///
///    See also: ~
///      • |nvim_buf_set_mark()|
///      • |nvim_del_mark()|
///
async fn buf_del_mark(&self, buffer: Buffer, name: &str, ) -> error::Result<Boolean> {
	self.call_fn("nvim_buf_del_mark".into(), (buffer, name, )).await
}
///                                                         *nvim_buf_set_mark()*
///nvim_buf_set_mark({buffer}, {name}, {line}, {col}, {opts})
///    Sets a named mark in the given buffer, all marks are allowed
///    file/uppercase, visual, last change, etc. See |mark-motions|.
///
///    Marks are (1,0)-indexed. |api-indexing|
///
///    Note: ~
///      • Passing 0 as line deletes the mark
///
///    Attributes: ~
///        Since: 0.6.0
///
///    Parameters: ~
///      • {buffer}  Buffer to set the mark on
///      • {name}    Mark name
///      • {line}    Line number
///      • {col}     Column/row number
///      • {opts}    Optional parameters. Reserved for future use.
///
///    Return: ~
///        true if the mark was set, else false.
///
///    See also: ~
///      • |nvim_buf_del_mark()|
///      • |nvim_buf_get_mark()|
///
async fn buf_set_mark(&self, buffer: Buffer, name: &str, line: Integer, col: Integer, opts: impl Serialize, ) -> error::Result<Boolean> {
	self.call_fn("nvim_buf_set_mark".into(), (buffer, name, line, col, opts, )).await
}
///                                                         *nvim_buf_set_mark()*
///nvim_buf_set_mark({buffer}, {name}, {line}, {col}, {opts})
///    Sets a named mark in the given buffer, all marks are allowed
///    file/uppercase, visual, last change, etc. See |mark-motions|.
///
///    Marks are (1,0)-indexed. |api-indexing|
///
///    Note: ~
///      • Passing 0 as line deletes the mark
///
///    Attributes: ~
///        Since: 0.6.0
///
///    Parameters: ~
///      • {buffer}  Buffer to set the mark on
///      • {name}    Mark name
///      • {line}    Line number
///      • {col}     Column/row number
///      • {opts}    Optional parameters. Reserved for future use.
///
///    Return: ~
///        true if the mark was set, else false.
///
///    See also: ~
///      • |nvim_buf_del_mark()|
///      • |nvim_buf_get_mark()|
///
async fn buf_set_mark_wv(&self, buffer: Buffer, name: String, line: Integer, col: Integer, opts: Dict, ) -> error::Result<Boolean> {
	self.call_fn_wv("nvim_buf_set_mark".into(), (buffer, name, line, col, opts, )).await
}
///nvim_buf_get_mark({buffer}, {name})                      *nvim_buf_get_mark()*
///    Returns a `(row,col)` tuple representing the position of the named mark.
///    "End of line" column position is returned as |v:maxcol| (big number). See
///    |mark-motions|.
///
///    Marks are (1,0)-indexed. |api-indexing|
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {name}    Mark name
///
///    Return: ~
///        (row, col) tuple, (0, 0) if the mark is not set, or is an
///        uppercase/file mark set in another buffer.
///
///    See also: ~
///      • |nvim_buf_set_mark()|
///      • |nvim_buf_del_mark()|
///
async fn buf_get_mark(&self, buffer: Buffer, name: &str, ) -> error::Result<Vec<Integer>> {
	self.call_fn("nvim_buf_get_mark".into(), (buffer, name, )).await
}
///nvim_parse_cmd({str}, {opts})                               *nvim_parse_cmd()*
///    Parse command line.
///
///    Doesn't check the validity of command arguments.
///
///    Attributes: ~
///        |api-fast|
///        Since: 0.8.0
///
///    Parameters: ~
///      • {str}   Command line string to parse. Cannot contain "\n".
///      • {opts}  Optional parameters. Reserved for future use.
///
///    Return: ~
///        Dict containing command information, with these keys:
///        • cmd: (string) Command name.
///        • range: (array) (optional) Command range (<line1> <line2>). Omitted
///          if command doesn't accept a range. Otherwise, has no elements if no
///          range was specified, one element if only a single range item was
///          specified, or two elements if both range items were specified.
///        • count: (number) (optional) Command <count>. Omitted if command
///          cannot take a count.
///        • reg: (string) (optional) Command <register>. Omitted if command
///          cannot take a register.
///        • bang: (boolean) Whether command contains a <bang> (!) modifier.
///        • args: (array) Command arguments.
///        • addr: (string) Value of |:command-addr|. Uses short name or "line"
///          for -addr=lines.
///        • nargs: (string) Value of |:command-nargs|.
///        • nextcmd: (string) Next command if there are multiple commands
///          separated by a |:bar|. Empty if there isn't a next command.
///        • magic: (dict) Which characters have special meaning in the command
///          arguments.
///          • file: (boolean) The command expands filenames. Which means
///            characters such as "%", "#" and wildcards are expanded.
///          • bar: (boolean) The "|" character is treated as a command separator
///            and the double quote character (") is treated as the start of a
///            comment.
///        • mods: (dict) |:command-modifiers|.
///          • filter: (dict) |:filter|.
///            • pattern: (string) Filter pattern. Empty string if there is no
///              filter.
///            • force: (boolean) Whether filter is inverted or not.
///          • silent: (boolean) |:silent|.
///          • emsg_silent: (boolean) |:silent!|.
///          • unsilent: (boolean) |:unsilent|.
///          • sandbox: (boolean) |:sandbox|.
///          • noautocmd: (boolean) |:noautocmd|.
///          • browse: (boolean) |:browse|.
///          • confirm: (boolean) |:confirm|.
///          • hide: (boolean) |:hide|.
///          • horizontal: (boolean) |:horizontal|.
///          • keepalt: (boolean) |:keepalt|.
///          • keepjumps: (boolean) |:keepjumps|.
///          • keepmarks: (boolean) |:keepmarks|.
///          • keeppatterns: (boolean) |:keeppatterns|.
///          • lockmarks: (boolean) |:lockmarks|.
///          • noswapfile: (boolean) |:noswapfile|.
///          • tab: (integer) |:tab|. -1 when omitted.
///          • verbose: (integer) |:verbose|. -1 when omitted.
///          • vertical: (boolean) |:vertical|.
///          • split: (string) Split modifier string, is an empty string when
///            there's no split modifier. If there is a split modifier it can be
///            one of:
///            • "aboveleft": |:aboveleft|.
///            • "belowright": |:belowright|.
///            • "topleft": |:topleft|.
///            • "botright": |:botright|.
///
///
///==============================================================================
async fn parse_cmd<D: Deserialize<'static>>(&self, str: &str, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_parse_cmd".into(), (str, opts, )).await
}
///nvim_parse_cmd({str}, {opts})                               *nvim_parse_cmd()*
///    Parse command line.
///
///    Doesn't check the validity of command arguments.
///
///    Attributes: ~
///        |api-fast|
///        Since: 0.8.0
///
///    Parameters: ~
///      • {str}   Command line string to parse. Cannot contain "\n".
///      • {opts}  Optional parameters. Reserved for future use.
///
///    Return: ~
///        Dict containing command information, with these keys:
///        • cmd: (string) Command name.
///        • range: (array) (optional) Command range (<line1> <line2>). Omitted
///          if command doesn't accept a range. Otherwise, has no elements if no
///          range was specified, one element if only a single range item was
///          specified, or two elements if both range items were specified.
///        • count: (number) (optional) Command <count>. Omitted if command
///          cannot take a count.
///        • reg: (string) (optional) Command <register>. Omitted if command
///          cannot take a register.
///        • bang: (boolean) Whether command contains a <bang> (!) modifier.
///        • args: (array) Command arguments.
///        • addr: (string) Value of |:command-addr|. Uses short name or "line"
///          for -addr=lines.
///        • nargs: (string) Value of |:command-nargs|.
///        • nextcmd: (string) Next command if there are multiple commands
///          separated by a |:bar|. Empty if there isn't a next command.
///        • magic: (dict) Which characters have special meaning in the command
///          arguments.
///          • file: (boolean) The command expands filenames. Which means
///            characters such as "%", "#" and wildcards are expanded.
///          • bar: (boolean) The "|" character is treated as a command separator
///            and the double quote character (") is treated as the start of a
///            comment.
///        • mods: (dict) |:command-modifiers|.
///          • filter: (dict) |:filter|.
///            • pattern: (string) Filter pattern. Empty string if there is no
///              filter.
///            • force: (boolean) Whether filter is inverted or not.
///          • silent: (boolean) |:silent|.
///          • emsg_silent: (boolean) |:silent!|.
///          • unsilent: (boolean) |:unsilent|.
///          • sandbox: (boolean) |:sandbox|.
///          • noautocmd: (boolean) |:noautocmd|.
///          • browse: (boolean) |:browse|.
///          • confirm: (boolean) |:confirm|.
///          • hide: (boolean) |:hide|.
///          • horizontal: (boolean) |:horizontal|.
///          • keepalt: (boolean) |:keepalt|.
///          • keepjumps: (boolean) |:keepjumps|.
///          • keepmarks: (boolean) |:keepmarks|.
///          • keeppatterns: (boolean) |:keeppatterns|.
///          • lockmarks: (boolean) |:lockmarks|.
///          • noswapfile: (boolean) |:noswapfile|.
///          • tab: (integer) |:tab|. -1 when omitted.
///          • verbose: (integer) |:verbose|. -1 when omitted.
///          • vertical: (boolean) |:vertical|.
///          • split: (string) Split modifier string, is an empty string when
///            there's no split modifier. If there is a split modifier it can be
///            one of:
///            • "aboveleft": |:aboveleft|.
///            • "belowright": |:belowright|.
///            • "topleft": |:topleft|.
///            • "botright": |:botright|.
///
///
///==============================================================================
async fn parse_cmd_wv(&self, str: String, opts: Dict, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_parse_cmd".into(), (str, opts, )).await
}
///nvim_cmd({cmd}, {opts})                                           *nvim_cmd()*
///    Executes an Ex command.
///
///    Unlike |nvim_command()| this command takes a structured Dict instead of a
///    String. This allows for easier construction and manipulation of an Ex
///    command. This also allows for things such as having spaces inside a
///    command argument, expanding filenames in a command that otherwise doesn't
///    expand filenames, etc. Command arguments may also be Number, Boolean or
///    String.
///
///    The first argument may also be used instead of count for commands that
///    support it in order to make their usage simpler with |vim.cmd()|. For
///    example, instead of `vim.cmd.bdelete{ count = 2 }`, you may do
///    `vim.cmd.bdelete(2)`.
///
///    On execution error: fails with Vimscript error, updates v:errmsg.
///
///    Attributes: ~
///        Since: 0.8.0
///
///    Parameters: ~
///      • {cmd}   Command to execute. Must be a Dict that can contain the same
///                values as the return value of |nvim_parse_cmd()| except
///                "addr", "nargs" and "nextcmd" which are ignored if provided.
///                All values except for "cmd" are optional.
///      • {opts}  Optional parameters.
///                • output: (boolean, default false) Whether to return command
///                  output.
///
///    Return: ~
///        Command output (non-error, non-shell |:!|) if `output` is true, else
///        empty string.
///
///    See also: ~
///      • |nvim_exec2()|
///      • |nvim_command()|
///
async fn cmd(&self, cmd: impl Serialize, opts: impl Serialize, ) -> error::Result<String> {
	self.call_fn("nvim_cmd".into(), (cmd, opts, )).await
}
///nvim_cmd({cmd}, {opts})                                           *nvim_cmd()*
///    Executes an Ex command.
///
///    Unlike |nvim_command()| this command takes a structured Dict instead of a
///    String. This allows for easier construction and manipulation of an Ex
///    command. This also allows for things such as having spaces inside a
///    command argument, expanding filenames in a command that otherwise doesn't
///    expand filenames, etc. Command arguments may also be Number, Boolean or
///    String.
///
///    The first argument may also be used instead of count for commands that
///    support it in order to make their usage simpler with |vim.cmd()|. For
///    example, instead of `vim.cmd.bdelete{ count = 2 }`, you may do
///    `vim.cmd.bdelete(2)`.
///
///    On execution error: fails with Vimscript error, updates v:errmsg.
///
///    Attributes: ~
///        Since: 0.8.0
///
///    Parameters: ~
///      • {cmd}   Command to execute. Must be a Dict that can contain the same
///                values as the return value of |nvim_parse_cmd()| except
///                "addr", "nargs" and "nextcmd" which are ignored if provided.
///                All values except for "cmd" are optional.
///      • {opts}  Optional parameters.
///                • output: (boolean, default false) Whether to return command
///                  output.
///
///    Return: ~
///        Command output (non-error, non-shell |:!|) if `output` is true, else
///        empty string.
///
///    See also: ~
///      • |nvim_exec2()|
///      • |nvim_command()|
///
async fn cmd_wv(&self, cmd: Dict, opts: Dict, ) -> error::Result<String> {
	self.call_fn_wv("nvim_cmd".into(), (cmd, opts, )).await
}
///                                                  *nvim_create_user_command()*
///nvim_create_user_command({name}, {command}, {opts})
///    Creates a global |user-commands| command.
///
///    For Lua usage see |lua-guide-commands-create|.
///
///    Example: >vim
///        :call nvim_create_user_command('SayHello', 'echo "Hello world!"', {'bang': v:true})
///        :SayHello
///        Hello world!
///<
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {name}     Name of the new user command. Must begin with an uppercase
///                   letter.
///      • {command}  Replacement command to execute when this user command is
///                   executed. When called from Lua, the command can also be a
///                   Lua function. The function is called with a single table
///                   argument that contains the following keys:
///                   • name: (string) Command name
///                   • args: (string) The args passed to the command, if any
///                     <args>
///                   • fargs: (table) The args split by unescaped whitespace
///                     (when more than one argument is allowed), if any <f-args>
///                   • nargs: (string) Number of arguments |:command-nargs|
///                   • bang: (boolean) "true" if the command was executed with a
///                     ! modifier <bang>
///                   • line1: (number) The starting line of the command range
///                     <line1>
///                   • line2: (number) The final line of the command range
///                     <line2>
///                   • range: (number) The number of items in the command range:
///                     0, 1, or 2 <range>
///                   • count: (number) Any count supplied <count>
///                   • reg: (string) The optional register, if specified <reg>
///                   • mods: (string) Command modifiers, if any <mods>
///                   • smods: (table) Command modifiers in a structured format.
///                     Has the same structure as the "mods" key of
///                     |nvim_parse_cmd()|.
///      • {opts}     Optional |command-attributes|.
///                   • Set boolean attributes such as |:command-bang| or
///                     |:command-bar| to true (but not |:command-buffer|, use
///                     |nvim_buf_create_user_command()| instead).
///                   • "complete" |:command-complete| also accepts a Lua
///                     function which works like
///                     |:command-completion-customlist|.
///                   • Other parameters:
///                     • desc: (string) Used for listing the command when a Lua
///                       function is used for {command}.
///                     • force: (boolean, default true) Override any previous
///                       definition.
///                     • preview: (function) Preview callback for 'inccommand'
///                       |:command-preview|
///
async fn create_user_command(&self, name: &str, command: impl Serialize, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_create_user_command".into(), (name, command, opts, )).await
}
///                                                  *nvim_create_user_command()*
///nvim_create_user_command({name}, {command}, {opts})
///    Creates a global |user-commands| command.
///
///    For Lua usage see |lua-guide-commands-create|.
///
///    Example: >vim
///        :call nvim_create_user_command('SayHello', 'echo "Hello world!"', {'bang': v:true})
///        :SayHello
///        Hello world!
///<
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {name}     Name of the new user command. Must begin with an uppercase
///                   letter.
///      • {command}  Replacement command to execute when this user command is
///                   executed. When called from Lua, the command can also be a
///                   Lua function. The function is called with a single table
///                   argument that contains the following keys:
///                   • name: (string) Command name
///                   • args: (string) The args passed to the command, if any
///                     <args>
///                   • fargs: (table) The args split by unescaped whitespace
///                     (when more than one argument is allowed), if any <f-args>
///                   • nargs: (string) Number of arguments |:command-nargs|
///                   • bang: (boolean) "true" if the command was executed with a
///                     ! modifier <bang>
///                   • line1: (number) The starting line of the command range
///                     <line1>
///                   • line2: (number) The final line of the command range
///                     <line2>
///                   • range: (number) The number of items in the command range:
///                     0, 1, or 2 <range>
///                   • count: (number) Any count supplied <count>
///                   • reg: (string) The optional register, if specified <reg>
///                   • mods: (string) Command modifiers, if any <mods>
///                   • smods: (table) Command modifiers in a structured format.
///                     Has the same structure as the "mods" key of
///                     |nvim_parse_cmd()|.
///      • {opts}     Optional |command-attributes|.
///                   • Set boolean attributes such as |:command-bang| or
///                     |:command-bar| to true (but not |:command-buffer|, use
///                     |nvim_buf_create_user_command()| instead).
///                   • "complete" |:command-complete| also accepts a Lua
///                     function which works like
///                     |:command-completion-customlist|.
///                   • Other parameters:
///                     • desc: (string) Used for listing the command when a Lua
///                       function is used for {command}.
///                     • force: (boolean, default true) Override any previous
///                       definition.
///                     • preview: (function) Preview callback for 'inccommand'
///                       |:command-preview|
///
async fn create_user_command_wv(&self, name: String, command: Object, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_create_user_command".into(), (name, command, opts, )).await
}
///nvim_del_user_command({name})                        *nvim_del_user_command()*
///    Delete a user-defined command.
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {name}  Name of the command to delete.
///
async fn del_user_command(&self, name: &str, ) -> error::Result<()> {
	self.call_fn("nvim_del_user_command".into(), (name, )).await
}
///                                              *nvim_buf_create_user_command()*
///nvim_buf_create_user_command({buffer}, {name}, {command}, {opts})
///    Creates a buffer-local command |user-commands|.
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {buffer}   Buffer id, or 0 for current buffer.
///
///    See also: ~
///      • nvim_create_user_command
///
async fn buf_create_user_command(&self, buffer: Buffer, name: &str, command: impl Serialize, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_buf_create_user_command".into(), (buffer, name, command, opts, )).await
}
///                                              *nvim_buf_create_user_command()*
///nvim_buf_create_user_command({buffer}, {name}, {command}, {opts})
///    Creates a buffer-local command |user-commands|.
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {buffer}   Buffer id, or 0 for current buffer.
///
///    See also: ~
///      • nvim_create_user_command
///
async fn buf_create_user_command_wv(&self, buffer: Buffer, name: String, command: Object, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_buf_create_user_command".into(), (buffer, name, command, opts, )).await
}
///                                                 *nvim_buf_del_user_command()*
///nvim_buf_del_user_command({buffer}, {name})
///    Delete a buffer-local user-defined command.
///
///    Only commands created with |:command-buffer| or
///    |nvim_buf_create_user_command()| can be deleted with this function.
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer.
///      • {name}    Name of the command to delete.
///
async fn buf_del_user_command(&self, buffer: Buffer, name: &str, ) -> error::Result<()> {
	self.call_fn("nvim_buf_del_user_command".into(), (buffer, name, )).await
}
///nvim_get_commands({opts})                                *nvim_get_commands()*
///    Gets a map of global (non-buffer-local) Ex commands.
///
///    Currently only |user-commands| are supported, not builtin Ex commands.
///
///    Attributes: ~
///        Since: 0.3.0
///
///    Parameters: ~
///      • {opts}  Optional parameters. Currently only supports {"builtin":false}
///
///    Return: ~
///        Map of maps describing commands.
///
///    See also: ~
///      • |nvim_get_all_options_info()|
///
async fn get_commands<D: Deserialize<'static>>(&self, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_get_commands".into(), (opts, )).await
}
///nvim_get_commands({opts})                                *nvim_get_commands()*
///    Gets a map of global (non-buffer-local) Ex commands.
///
///    Currently only |user-commands| are supported, not builtin Ex commands.
///
///    Attributes: ~
///        Since: 0.3.0
///
///    Parameters: ~
///      • {opts}  Optional parameters. Currently only supports {"builtin":false}
///
///    Return: ~
///        Map of maps describing commands.
///
///    See also: ~
///      • |nvim_get_all_options_info()|
///
async fn get_commands_wv(&self, opts: Dict, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_get_commands".into(), (opts, )).await
}
///nvim_buf_get_commands({buffer}, {opts})              *nvim_buf_get_commands()*
///    Gets a map of buffer-local |user-commands|.
///
///    Attributes: ~
///        Since: 0.3.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {opts}    Optional parameters. Currently not used.
///
///    Return: ~
///        Map of maps describing commands.
///
async fn buf_get_commands<D: Deserialize<'static>>(&self, buffer: Buffer, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_buf_get_commands".into(), (buffer, opts, )).await
}
///nvim_buf_get_commands({buffer}, {opts})              *nvim_buf_get_commands()*
///    Gets a map of buffer-local |user-commands|.
///
///    Attributes: ~
///        Since: 0.3.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {opts}    Optional parameters. Currently not used.
///
///    Return: ~
///        Map of maps describing commands.
///
async fn buf_get_commands_wv(&self, buffer: Buffer, opts: Dict, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_buf_get_commands".into(), (buffer, opts, )).await
}
///nvim_create_namespace({name})                        *nvim_create_namespace()*
async fn create_namespace(&self, name: &str, ) -> error::Result<Integer> {
	self.call_fn("nvim_create_namespace".into(), (name, )).await
}
///nvim_get_namespaces()                                  *nvim_get_namespaces()*
///    Gets existing, non-anonymous |namespace|s.
///
///    Attributes: ~
///        Since: 0.3.2
///
///    Return: ~
///        dict that maps from names to namespace ids.
///
async fn get_namespaces<D: Deserialize<'static>>(&self, ) -> error::Result<D> {
	self.call_fn("nvim_get_namespaces".into(), [();0]).await
}
///nvim_get_namespaces()                                  *nvim_get_namespaces()*
///    Gets existing, non-anonymous |namespace|s.
///
///    Attributes: ~
///        Since: 0.3.2
///
///    Return: ~
///        dict that maps from names to namespace ids.
///
async fn get_namespaces_wv(&self, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_get_namespaces".into(), [();0]).await
}
///                                                *nvim_buf_get_extmark_by_id()*
///nvim_buf_get_extmark_by_id({buffer}, {ns_id}, {id}, {opts})
///    Gets the position (0-indexed) of an |extmark|.
///
///    Attributes: ~
///        Since: 0.5.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {ns_id}   Namespace id from |nvim_create_namespace()|
///      • {id}      Extmark id
///      • {opts}    Optional parameters. Keys:
///                  • details: Whether to include the details dict
///                  • hl_name: Whether to include highlight group name instead
///                    of id, true if omitted
///
///    Return: ~
///        0-indexed (row, col) tuple or empty list () if extmark id was absent
///
async fn buf_get_extmark_by_id(&self, buffer: Buffer, ns_id: Integer, id: Integer, opts: impl Serialize, ) -> error::Result<Vec<Integer>> {
	self.call_fn("nvim_buf_get_extmark_by_id".into(), (buffer, ns_id, id, opts, )).await
}
///                                                *nvim_buf_get_extmark_by_id()*
///nvim_buf_get_extmark_by_id({buffer}, {ns_id}, {id}, {opts})
///    Gets the position (0-indexed) of an |extmark|.
///
///    Attributes: ~
///        Since: 0.5.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {ns_id}   Namespace id from |nvim_create_namespace()|
///      • {id}      Extmark id
///      • {opts}    Optional parameters. Keys:
///                  • details: Whether to include the details dict
///                  • hl_name: Whether to include highlight group name instead
///                    of id, true if omitted
///
///    Return: ~
///        0-indexed (row, col) tuple or empty list () if extmark id was absent
///
async fn buf_get_extmark_by_id_wv(&self, buffer: Buffer, ns_id: Integer, id: Integer, opts: Dict, ) -> error::Result<Vec<Integer>> {
	self.call_fn_wv("nvim_buf_get_extmark_by_id".into(), (buffer, ns_id, id, opts, )).await
}
///                                                     *nvim_buf_get_extmarks()*
///nvim_buf_get_extmarks({buffer}, {ns_id}, {start}, {end}, {opts})
///    Gets |extmarks| in "traversal order" from a |charwise| region defined by
///    buffer positions (inclusive, 0-indexed |api-indexing|).
///
///    Region can be given as (row,col) tuples, or valid extmark ids (whose
///    positions define the bounds). 0 and -1 are understood as (0,0) and (-1,-1)
///    respectively, thus the following are equivalent: >lua
///        vim.api.nvim_buf_get_extmarks(0, my_ns, 0, -1, {})
///        vim.api.nvim_buf_get_extmarks(0, my_ns, {0,0}, {-1,-1}, {})
///<
///
///    If `end` is less than `start`, marks are returned in reverse order.
///    (Useful with `limit`, to get the first marks prior to a given position.)
///
///    Note: For a reverse range, `limit` does not actually affect the traversed
///    range, just how many marks are returned
///
///    Note: when using extmark ranges (marks with a end_row/end_col position)
///    the `overlap` option might be useful. Otherwise only the start position of
///    an extmark will be considered.
///
///    Note: legacy signs placed through the |:sign| commands are implemented as
///    extmarks and will show up here. Their details array will contain a
///    `sign_name` field.
///
///    Example: >lua
///        local api = vim.api
///        local pos = api.nvim_win_get_cursor(0)
///        local ns  = api.nvim_create_namespace('my-plugin')
///        -- Create new extmark at line 1, column 1.
///        local m1  = api.nvim_buf_set_extmark(0, ns, 0, 0, {})
///        -- Create new extmark at line 3, column 1.
///        local m2  = api.nvim_buf_set_extmark(0, ns, 2, 0, {})
///        -- Get extmarks only from line 3.
///        local ms  = api.nvim_buf_get_extmarks(0, ns, {2,0}, {2,0}, {})
///        -- Get all marks in this buffer + namespace.
///        local all = api.nvim_buf_get_extmarks(0, ns, 0, -1, {})
///        vim.print(ms)
///<
///
///    Attributes: ~
///        Since: 0.5.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {ns_id}   Namespace id from |nvim_create_namespace()| or -1 for all
///                  namespaces
///      • {start}   Start of range: a 0-indexed (row, col) or valid extmark id
///                  (whose position defines the bound). |api-indexing|
///      • {end}     End of range (inclusive): a 0-indexed (row, col) or valid
///                  extmark id (whose position defines the bound).
///                  |api-indexing|
///      • {opts}    Optional parameters. Keys:
///                  • limit: Maximum number of marks to return
///                  • details: Whether to include the details dict
///                  • hl_name: Whether to include highlight group name instead
///                    of id, true if omitted
///                  • overlap: Also include marks which overlap the range, even
///                    if their start position is less than `start`
///                  • type: Filter marks by type: "highlight", "sign",
///                    "virt_text" and "virt_lines"
///
///    Return: ~
///        List of `[extmark_id, row, col]` tuples in "traversal order".
///
async fn buf_get_extmarks<D: Deserialize<'static>>(&self, buffer: Buffer, ns_id: Integer, start: impl Serialize, end: impl Serialize, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_buf_get_extmarks".into(), (buffer, ns_id, start, end, opts, )).await
}
///                                                     *nvim_buf_get_extmarks()*
///nvim_buf_get_extmarks({buffer}, {ns_id}, {start}, {end}, {opts})
///    Gets |extmarks| in "traversal order" from a |charwise| region defined by
///    buffer positions (inclusive, 0-indexed |api-indexing|).
///
///    Region can be given as (row,col) tuples, or valid extmark ids (whose
///    positions define the bounds). 0 and -1 are understood as (0,0) and (-1,-1)
///    respectively, thus the following are equivalent: >lua
///        vim.api.nvim_buf_get_extmarks(0, my_ns, 0, -1, {})
///        vim.api.nvim_buf_get_extmarks(0, my_ns, {0,0}, {-1,-1}, {})
///<
///
///    If `end` is less than `start`, marks are returned in reverse order.
///    (Useful with `limit`, to get the first marks prior to a given position.)
///
///    Note: For a reverse range, `limit` does not actually affect the traversed
///    range, just how many marks are returned
///
///    Note: when using extmark ranges (marks with a end_row/end_col position)
///    the `overlap` option might be useful. Otherwise only the start position of
///    an extmark will be considered.
///
///    Note: legacy signs placed through the |:sign| commands are implemented as
///    extmarks and will show up here. Their details array will contain a
///    `sign_name` field.
///
///    Example: >lua
///        local api = vim.api
///        local pos = api.nvim_win_get_cursor(0)
///        local ns  = api.nvim_create_namespace('my-plugin')
///        -- Create new extmark at line 1, column 1.
///        local m1  = api.nvim_buf_set_extmark(0, ns, 0, 0, {})
///        -- Create new extmark at line 3, column 1.
///        local m2  = api.nvim_buf_set_extmark(0, ns, 2, 0, {})
///        -- Get extmarks only from line 3.
///        local ms  = api.nvim_buf_get_extmarks(0, ns, {2,0}, {2,0}, {})
///        -- Get all marks in this buffer + namespace.
///        local all = api.nvim_buf_get_extmarks(0, ns, 0, -1, {})
///        vim.print(ms)
///<
///
///    Attributes: ~
///        Since: 0.5.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {ns_id}   Namespace id from |nvim_create_namespace()| or -1 for all
///                  namespaces
///      • {start}   Start of range: a 0-indexed (row, col) or valid extmark id
///                  (whose position defines the bound). |api-indexing|
///      • {end}     End of range (inclusive): a 0-indexed (row, col) or valid
///                  extmark id (whose position defines the bound).
///                  |api-indexing|
///      • {opts}    Optional parameters. Keys:
///                  • limit: Maximum number of marks to return
///                  • details: Whether to include the details dict
///                  • hl_name: Whether to include highlight group name instead
///                    of id, true if omitted
///                  • overlap: Also include marks which overlap the range, even
///                    if their start position is less than `start`
///                  • type: Filter marks by type: "highlight", "sign",
///                    "virt_text" and "virt_lines"
///
///    Return: ~
///        List of `[extmark_id, row, col]` tuples in "traversal order".
///
async fn buf_get_extmarks_wv(&self, buffer: Buffer, ns_id: Integer, start: Object, end: Object, opts: Dict, ) -> error::Result<Array> {
	self.call_fn_wv("nvim_buf_get_extmarks".into(), (buffer, ns_id, start, end, opts, )).await
}
///                                                      *nvim_buf_set_extmark()*
///nvim_buf_set_extmark({buffer}, {ns_id}, {line}, {col}, {opts})
///    Creates or updates an |extmark|.
///
///    By default a new extmark is created when no id is passed in, but it is
///    also possible to create a new mark by passing in a previously unused id or
///    move an existing mark by passing in its id. The caller must then keep
///    track of existing and unused ids itself. (Useful over RPC, to avoid
///    waiting for the return value.)
///
///    Using the optional arguments, it is possible to use this to highlight a
///    range of text, and also to associate virtual text to the mark.
///
///    If present, the position defined by `end_col` and `end_row` should be
///    after the start position in order for the extmark to cover a range. An
///    earlier end position is not an error, but then it behaves like an empty
///    range (no highlighting).
///
///    Attributes: ~
///        Since: 0.5.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {ns_id}   Namespace id from |nvim_create_namespace()|
///      • {line}    Line where to place the mark, 0-based. |api-indexing|
///      • {col}     Column where to place the mark, 0-based. |api-indexing|
///      • {opts}    Optional parameters.
///                  • id : id of the extmark to edit.
///                  • end_row : ending line of the mark, 0-based inclusive.
///                  • end_col : ending col of the mark, 0-based exclusive.
///                  • hl_group : highlight group used for the text range. This
///                    and below highlight groups can be supplied either as a
///                    string or as an integer, the latter of which can be
///                    obtained using |nvim_get_hl_id_by_name()|.
///                    Multiple highlight groups can be stacked by passing an
///                    array (highest priority last).
///                  • hl_eol : when true, for a multiline highlight covering the
///                    EOL of a line, continue the highlight for the rest of the
///                    screen line (just like for diff and cursorline highlight).
///                  • virt_text : virtual text to link to this mark. A list of
///                    `[text, highlight]` tuples, each representing a text chunk
///                    with specified highlight. `highlight` element can either
///                    be a single highlight group, or an array of multiple
///                    highlight groups that will be stacked (highest priority
///                    last).
///                  • virt_text_pos : position of virtual text. Possible values:
///                    • "eol": right after eol character (default).
///                    • "eol_right_align": display right aligned in the window
///                      unless the virtual text is longer than the space
///                      available. If the virtual text is too long, it is
///                      truncated to fit in the window after the EOL character.
///                      If the line is wrapped, the virtual text is shown after
///                      the end of the line rather than the previous screen
///                      line.
///                    • "overlay": display over the specified column, without
///                      shifting the underlying text.
///                    • "right_align": display right aligned in the window.
///                    • "inline": display at the specified column, and shift the
///                      buffer text to the right as needed.
///                  • virt_text_win_col : position the virtual text at a fixed
///                    window column (starting from the first text column of the
///                    screen line) instead of "virt_text_pos".
///                  • virt_text_hide : hide the virtual text when the background
///                    text is selected or hidden because of scrolling with
///                    'nowrap' or 'smoothscroll'. Currently only affects
///                    "overlay" virt_text.
///                  • virt_text_repeat_linebreak : repeat the virtual text on
///                    wrapped lines.
///                  • hl_mode : control how highlights are combined with the
///                    highlights of the text. Currently only affects virt_text
///                    highlights, but might affect `hl_group` in later versions.
///                    • "replace": only show the virt_text color. This is the
///                      default.
///                    • "combine": combine with background text color.
///                    • "blend": blend with background text color. Not supported
///                      for "inline" virt_text.
///                  • virt_lines : virtual lines to add next to this mark This
///                    should be an array over lines, where each line in turn is
///                    an array over `[text, highlight]` tuples. In general,
///                    buffer and window options do not affect the display of the
///                    text. In particular 'wrap' and 'linebreak' options do not
///                    take effect, so the number of extra screen lines will
///                    always match the size of the array. However the 'tabstop'
///                    buffer option is still used for hard tabs. By default
///                    lines are placed below the buffer line containing the
///                    mark.
///                  • virt_lines_above: place virtual lines above instead.
///                  • virt_lines_leftcol: Place virtual lines in the leftmost
///                    column of the window, bypassing sign and number columns.
///                  • virt_lines_overflow: controls how to handle virtual lines
///                    wider than the window. Currently takes the one of the
///                    following values:
///                    • "trunc": truncate virtual lines on the right (default).
///                    • "scroll": virtual lines can scroll horizontally with
///                      'nowrap', otherwise the same as "trunc".
///                  • ephemeral : for use with |nvim_set_decoration_provider()|
///                    callbacks. The mark will only be used for the current
///                    redraw cycle, and not be permanently stored in the buffer.
///                  • right_gravity : boolean that indicates the direction the
///                    extmark will be shifted in when new text is inserted (true
///                    for right, false for left). Defaults to true.
///                  • end_right_gravity : boolean that indicates the direction
///                    the extmark end position (if it exists) will be shifted in
///                    when new text is inserted (true for right, false for
///                    left). Defaults to false.
///                  • undo_restore : Restore the exact position of the mark if
///                    text around the mark was deleted and then restored by
///                    undo. Defaults to true.
///                  • invalidate : boolean that indicates whether to hide the
///                    extmark if the entirety of its range is deleted. For
///                    hidden marks, an "invalid" key is added to the "details"
///                    array of |nvim_buf_get_extmarks()| and family. If
///                    "undo_restore" is false, the extmark is deleted instead.
///                  • priority: a priority value for the highlight group, sign
///                    attribute or virtual text. For virtual text, item with
///                    highest priority is drawn last. For example treesitter
///                    highlighting uses a value of 100.
///                  • strict: boolean that indicates extmark should not be
///                    placed if the line or column value is past the end of the
///                    buffer or end of the line respectively. Defaults to true.
///                  • sign_text: string of length 1-2 used to display in the
///                    sign column.
///                  • sign_hl_group: highlight group used for the sign column
///                    text.
///                  • number_hl_group: highlight group used for the number
///                    column.
///                  • line_hl_group: highlight group used for the whole line.
///                  • cursorline_hl_group: highlight group used for the sign
///                    column text when the cursor is on the same line as the
///                    mark and 'cursorline' is enabled.
///                  • conceal: string which should be either empty or a single
///                    character. Enable concealing similar to |:syn-conceal|.
///                    When a character is supplied it is used as |:syn-cchar|.
///                    "hl_group" is used as highlight for the cchar if provided,
///                    otherwise it defaults to |hl-Conceal|.
///                  • conceal_lines: string which should be empty. When
///                    provided, lines in the range are not drawn at all
///                    (according to 'conceallevel'); the next unconcealed line
///                    is drawn instead.
///                  • spell: boolean indicating that spell checking should be
///                    performed within this extmark
///                  • ui_watched: boolean that indicates the mark should be
///                    drawn by a UI. When set, the UI will receive win_extmark
///                    events. Note: the mark is positioned by virt_text
///                    attributes. Can be used together with virt_text.
///                  • url: A URL to associate with this extmark. In the TUI, the
///                    OSC 8 control sequence is used to generate a clickable
///                    hyperlink to this URL.
///
///    Return: ~
///        Id of the created/updated extmark
///
async fn buf_set_extmark(&self, buffer: Buffer, ns_id: Integer, line: Integer, col: Integer, opts: impl Serialize, ) -> error::Result<Integer> {
	self.call_fn("nvim_buf_set_extmark".into(), (buffer, ns_id, line, col, opts, )).await
}
///                                                      *nvim_buf_set_extmark()*
///nvim_buf_set_extmark({buffer}, {ns_id}, {line}, {col}, {opts})
///    Creates or updates an |extmark|.
///
///    By default a new extmark is created when no id is passed in, but it is
///    also possible to create a new mark by passing in a previously unused id or
///    move an existing mark by passing in its id. The caller must then keep
///    track of existing and unused ids itself. (Useful over RPC, to avoid
///    waiting for the return value.)
///
///    Using the optional arguments, it is possible to use this to highlight a
///    range of text, and also to associate virtual text to the mark.
///
///    If present, the position defined by `end_col` and `end_row` should be
///    after the start position in order for the extmark to cover a range. An
///    earlier end position is not an error, but then it behaves like an empty
///    range (no highlighting).
///
///    Attributes: ~
///        Since: 0.5.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {ns_id}   Namespace id from |nvim_create_namespace()|
///      • {line}    Line where to place the mark, 0-based. |api-indexing|
///      • {col}     Column where to place the mark, 0-based. |api-indexing|
///      • {opts}    Optional parameters.
///                  • id : id of the extmark to edit.
///                  • end_row : ending line of the mark, 0-based inclusive.
///                  • end_col : ending col of the mark, 0-based exclusive.
///                  • hl_group : highlight group used for the text range. This
///                    and below highlight groups can be supplied either as a
///                    string or as an integer, the latter of which can be
///                    obtained using |nvim_get_hl_id_by_name()|.
///                    Multiple highlight groups can be stacked by passing an
///                    array (highest priority last).
///                  • hl_eol : when true, for a multiline highlight covering the
///                    EOL of a line, continue the highlight for the rest of the
///                    screen line (just like for diff and cursorline highlight).
///                  • virt_text : virtual text to link to this mark. A list of
///                    `[text, highlight]` tuples, each representing a text chunk
///                    with specified highlight. `highlight` element can either
///                    be a single highlight group, or an array of multiple
///                    highlight groups that will be stacked (highest priority
///                    last).
///                  • virt_text_pos : position of virtual text. Possible values:
///                    • "eol": right after eol character (default).
///                    • "eol_right_align": display right aligned in the window
///                      unless the virtual text is longer than the space
///                      available. If the virtual text is too long, it is
///                      truncated to fit in the window after the EOL character.
///                      If the line is wrapped, the virtual text is shown after
///                      the end of the line rather than the previous screen
///                      line.
///                    • "overlay": display over the specified column, without
///                      shifting the underlying text.
///                    • "right_align": display right aligned in the window.
///                    • "inline": display at the specified column, and shift the
///                      buffer text to the right as needed.
///                  • virt_text_win_col : position the virtual text at a fixed
///                    window column (starting from the first text column of the
///                    screen line) instead of "virt_text_pos".
///                  • virt_text_hide : hide the virtual text when the background
///                    text is selected or hidden because of scrolling with
///                    'nowrap' or 'smoothscroll'. Currently only affects
///                    "overlay" virt_text.
///                  • virt_text_repeat_linebreak : repeat the virtual text on
///                    wrapped lines.
///                  • hl_mode : control how highlights are combined with the
///                    highlights of the text. Currently only affects virt_text
///                    highlights, but might affect `hl_group` in later versions.
///                    • "replace": only show the virt_text color. This is the
///                      default.
///                    • "combine": combine with background text color.
///                    • "blend": blend with background text color. Not supported
///                      for "inline" virt_text.
///                  • virt_lines : virtual lines to add next to this mark This
///                    should be an array over lines, where each line in turn is
///                    an array over `[text, highlight]` tuples. In general,
///                    buffer and window options do not affect the display of the
///                    text. In particular 'wrap' and 'linebreak' options do not
///                    take effect, so the number of extra screen lines will
///                    always match the size of the array. However the 'tabstop'
///                    buffer option is still used for hard tabs. By default
///                    lines are placed below the buffer line containing the
///                    mark.
///                  • virt_lines_above: place virtual lines above instead.
///                  • virt_lines_leftcol: Place virtual lines in the leftmost
///                    column of the window, bypassing sign and number columns.
///                  • virt_lines_overflow: controls how to handle virtual lines
///                    wider than the window. Currently takes the one of the
///                    following values:
///                    • "trunc": truncate virtual lines on the right (default).
///                    • "scroll": virtual lines can scroll horizontally with
///                      'nowrap', otherwise the same as "trunc".
///                  • ephemeral : for use with |nvim_set_decoration_provider()|
///                    callbacks. The mark will only be used for the current
///                    redraw cycle, and not be permanently stored in the buffer.
///                  • right_gravity : boolean that indicates the direction the
///                    extmark will be shifted in when new text is inserted (true
///                    for right, false for left). Defaults to true.
///                  • end_right_gravity : boolean that indicates the direction
///                    the extmark end position (if it exists) will be shifted in
///                    when new text is inserted (true for right, false for
///                    left). Defaults to false.
///                  • undo_restore : Restore the exact position of the mark if
///                    text around the mark was deleted and then restored by
///                    undo. Defaults to true.
///                  • invalidate : boolean that indicates whether to hide the
///                    extmark if the entirety of its range is deleted. For
///                    hidden marks, an "invalid" key is added to the "details"
///                    array of |nvim_buf_get_extmarks()| and family. If
///                    "undo_restore" is false, the extmark is deleted instead.
///                  • priority: a priority value for the highlight group, sign
///                    attribute or virtual text. For virtual text, item with
///                    highest priority is drawn last. For example treesitter
///                    highlighting uses a value of 100.
///                  • strict: boolean that indicates extmark should not be
///                    placed if the line or column value is past the end of the
///                    buffer or end of the line respectively. Defaults to true.
///                  • sign_text: string of length 1-2 used to display in the
///                    sign column.
///                  • sign_hl_group: highlight group used for the sign column
///                    text.
///                  • number_hl_group: highlight group used for the number
///                    column.
///                  • line_hl_group: highlight group used for the whole line.
///                  • cursorline_hl_group: highlight group used for the sign
///                    column text when the cursor is on the same line as the
///                    mark and 'cursorline' is enabled.
///                  • conceal: string which should be either empty or a single
///                    character. Enable concealing similar to |:syn-conceal|.
///                    When a character is supplied it is used as |:syn-cchar|.
///                    "hl_group" is used as highlight for the cchar if provided,
///                    otherwise it defaults to |hl-Conceal|.
///                  • conceal_lines: string which should be empty. When
///                    provided, lines in the range are not drawn at all
///                    (according to 'conceallevel'); the next unconcealed line
///                    is drawn instead.
///                  • spell: boolean indicating that spell checking should be
///                    performed within this extmark
///                  • ui_watched: boolean that indicates the mark should be
///                    drawn by a UI. When set, the UI will receive win_extmark
///                    events. Note: the mark is positioned by virt_text
///                    attributes. Can be used together with virt_text.
///                  • url: A URL to associate with this extmark. In the TUI, the
///                    OSC 8 control sequence is used to generate a clickable
///                    hyperlink to this URL.
///
///    Return: ~
///        Id of the created/updated extmark
///
async fn buf_set_extmark_wv(&self, buffer: Buffer, ns_id: Integer, line: Integer, col: Integer, opts: Dict, ) -> error::Result<Integer> {
	self.call_fn_wv("nvim_buf_set_extmark".into(), (buffer, ns_id, line, col, opts, )).await
}
///nvim_buf_del_extmark({buffer}, {ns_id}, {id})         *nvim_buf_del_extmark()*
///    Removes an |extmark|.
///
///    Attributes: ~
///        Since: 0.5.0
///
///    Parameters: ~
///      • {buffer}  Buffer id, or 0 for current buffer
///      • {ns_id}   Namespace id from |nvim_create_namespace()|
///      • {id}      Extmark id
///
///    Return: ~
///        true if the extmark was found, else false
///
async fn buf_del_extmark(&self, buffer: Buffer, ns_id: Integer, id: Integer, ) -> error::Result<Boolean> {
	self.call_fn("nvim_buf_del_extmark".into(), (buffer, ns_id, id, )).await
}
///                                                  *nvim_buf_clear_namespace()*
///nvim_buf_clear_namespace({buffer}, {ns_id}, {line_start}, {line_end})
///    Clears |namespace|d objects (highlights, |extmarks|, virtual text) from a
///    region.
///
///    Lines are 0-indexed. |api-indexing| To clear the namespace in the entire
///    buffer, specify line_start=0 and line_end=-1.
///
///    Attributes: ~
///        Since: 0.3.2
///
///    Parameters: ~
///      • {buffer}      Buffer id, or 0 for current buffer
///      • {ns_id}       Namespace to clear, or -1 to clear all namespaces.
///      • {line_start}  Start of range of lines to clear
///      • {line_end}    End of range of lines to clear (exclusive) or -1 to
///                      clear to end of buffer.
///
async fn buf_clear_namespace(&self, buffer: Buffer, ns_id: Integer, line_start: Integer, line_end: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_buf_clear_namespace".into(), (buffer, ns_id, line_start, line_end, )).await
}
///                                              *nvim_set_decoration_provider()*
///nvim_set_decoration_provider({ns_id}, {opts})
///    Set or change decoration provider for a |namespace|
///
///    This is a very general purpose interface for having Lua callbacks being
///    triggered during the redraw code.
///
///    The expected usage is to set |extmarks| for the currently redrawn buffer.
///    |nvim_buf_set_extmark()| can be called to add marks on a per-window or
///    per-lines basis. Use the `ephemeral` key to only use the mark for the
///    current screen redraw (the callback will be called again for the next
///    redraw).
///
///    Note: this function should not be called often. Rather, the callbacks
///    themselves can be used to throttle unneeded callbacks. the `on_start`
///    callback can return `false` to disable the provider until the next redraw.
///    Similarly, return `false` in `on_win` will skip the `on_line` calls for
///    that window (but any extmarks set in `on_win` will still be used). A
///    plugin managing multiple sources of decoration should ideally only set one
///    provider, and merge the sources internally. You can use multiple `ns_id`
///    for the extmarks set/modified inside the callback anyway.
///
///    Note: doing anything other than setting extmarks is considered
///    experimental. Doing things like changing options are not explicitly
///    forbidden, but is likely to have unexpected consequences (such as 100% CPU
///    consumption). Doing `vim.rpcnotify` should be OK, but `vim.rpcrequest` is
///    quite dubious for the moment.
///
///    Note: It is not allowed to remove or update extmarks in `on_line`
///    callbacks.
///
///    Attributes: ~
///        Lua |vim.api| only
///        Since: 0.5.0
///
///    Parameters: ~
///      • {ns_id}  Namespace id from |nvim_create_namespace()|
///      • {opts}   Table of callbacks:
///                 • on_start: called first on each screen redraw >
///                    ["start", tick]
///<
///                 • on_buf: called for each buffer being redrawn (once per
///                   edit, before window callbacks) >
///                    ["buf", bufnr, tick]
///<
///                 • on_win: called when starting to redraw a specific window. >
///                    ["win", winid, bufnr, toprow, botrow]
///<
///                 • on_line: called for each buffer line being redrawn. (The
///                   interaction with fold lines is subject to change) >
///                    ["line", winid, bufnr, row]
///<
///                 • on_end: called at the end of a redraw cycle >
///                    ["end", tick]
///<
///
async fn set_decoration_provider(&self, ns_id: Integer, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_set_decoration_provider".into(), (ns_id, opts, )).await
}
///                                              *nvim_set_decoration_provider()*
///nvim_set_decoration_provider({ns_id}, {opts})
///    Set or change decoration provider for a |namespace|
///
///    This is a very general purpose interface for having Lua callbacks being
///    triggered during the redraw code.
///
///    The expected usage is to set |extmarks| for the currently redrawn buffer.
///    |nvim_buf_set_extmark()| can be called to add marks on a per-window or
///    per-lines basis. Use the `ephemeral` key to only use the mark for the
///    current screen redraw (the callback will be called again for the next
///    redraw).
///
///    Note: this function should not be called often. Rather, the callbacks
///    themselves can be used to throttle unneeded callbacks. the `on_start`
///    callback can return `false` to disable the provider until the next redraw.
///    Similarly, return `false` in `on_win` will skip the `on_line` calls for
///    that window (but any extmarks set in `on_win` will still be used). A
///    plugin managing multiple sources of decoration should ideally only set one
///    provider, and merge the sources internally. You can use multiple `ns_id`
///    for the extmarks set/modified inside the callback anyway.
///
///    Note: doing anything other than setting extmarks is considered
///    experimental. Doing things like changing options are not explicitly
///    forbidden, but is likely to have unexpected consequences (such as 100% CPU
///    consumption). Doing `vim.rpcnotify` should be OK, but `vim.rpcrequest` is
///    quite dubious for the moment.
///
///    Note: It is not allowed to remove or update extmarks in `on_line`
///    callbacks.
///
///    Attributes: ~
///        Lua |vim.api| only
///        Since: 0.5.0
///
///    Parameters: ~
///      • {ns_id}  Namespace id from |nvim_create_namespace()|
///      • {opts}   Table of callbacks:
///                 • on_start: called first on each screen redraw >
///                    ["start", tick]
///<
///                 • on_buf: called for each buffer being redrawn (once per
///                   edit, before window callbacks) >
///                    ["buf", bufnr, tick]
///<
///                 • on_win: called when starting to redraw a specific window. >
///                    ["win", winid, bufnr, toprow, botrow]
///<
///                 • on_line: called for each buffer line being redrawn. (The
///                   interaction with fold lines is subject to change) >
///                    ["line", winid, bufnr, row]
///<
///                 • on_end: called at the end of a redraw cycle >
///                    ["end", tick]
///<
///
async fn set_decoration_provider_wv(&self, ns_id: Integer, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_set_decoration_provider".into(), (ns_id, opts, )).await
}
///nvim_get_option_value({name}, {opts})                *nvim_get_option_value()*
///    Gets the value of an option. The behavior of this function matches that of
///    |:set|: the local value of an option is returned if it exists; otherwise,
///    the global value is returned. Local values always correspond to the
///    current buffer or window, unless "buf" or "win" is set in {opts}.
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {name}  Option name
///      • {opts}  Optional parameters
///                • scope: One of "global" or "local". Analogous to |:setglobal|
///                  and |:setlocal|, respectively.
///                • win: |window-ID|. Used for getting window local options.
///                • buf: Buffer number. Used for getting buffer local options.
///                  Implies {scope} is "local".
///                • filetype: |filetype|. Used to get the default option for a
///                  specific filetype. Cannot be used with any other option.
///                  Note: this will trigger |ftplugin| and all |FileType|
///                  autocommands for the corresponding filetype.
///
///    Return: ~
///        Option value
///
async fn get_option_value<D: Deserialize<'static>>(&self, name: &str, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_get_option_value".into(), (name, opts, )).await
}
///nvim_get_option_value({name}, {opts})                *nvim_get_option_value()*
///    Gets the value of an option. The behavior of this function matches that of
///    |:set|: the local value of an option is returned if it exists; otherwise,
///    the global value is returned. Local values always correspond to the
///    current buffer or window, unless "buf" or "win" is set in {opts}.
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {name}  Option name
///      • {opts}  Optional parameters
///                • scope: One of "global" or "local". Analogous to |:setglobal|
///                  and |:setlocal|, respectively.
///                • win: |window-ID|. Used for getting window local options.
///                • buf: Buffer number. Used for getting buffer local options.
///                  Implies {scope} is "local".
///                • filetype: |filetype|. Used to get the default option for a
///                  specific filetype. Cannot be used with any other option.
///                  Note: this will trigger |ftplugin| and all |FileType|
///                  autocommands for the corresponding filetype.
///
///    Return: ~
///        Option value
///
async fn get_option_value_wv(&self, name: String, opts: Dict, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_get_option_value".into(), (name, opts, )).await
}
///                                                     *nvim_set_option_value()*
///nvim_set_option_value({name}, {value}, {opts})
///    Sets the value of an option. The behavior of this function matches that of
///    |:set|: for global-local options, both the global and local value are set
///    unless otherwise specified with {scope}.
///
///    Note the options {win} and {buf} cannot be used together.
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {name}   Option name
///      • {value}  New option value
///      • {opts}   Optional parameters
///                 • scope: One of "global" or "local". Analogous to
///                   |:setglobal| and |:setlocal|, respectively.
///                 • win: |window-ID|. Used for setting window local option.
///                 • buf: Buffer number. Used for setting buffer local option.
///
///
///==============================================================================
async fn set_option_value(&self, name: &str, value: impl Serialize, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_set_option_value".into(), (name, value, opts, )).await
}
///                                                     *nvim_set_option_value()*
///nvim_set_option_value({name}, {value}, {opts})
///    Sets the value of an option. The behavior of this function matches that of
///    |:set|: for global-local options, both the global and local value are set
///    unless otherwise specified with {scope}.
///
///    Note the options {win} and {buf} cannot be used together.
///
///    Attributes: ~
///        Since: 0.7.0
///
///    Parameters: ~
///      • {name}   Option name
///      • {value}  New option value
///      • {opts}   Optional parameters
///                 • scope: One of "global" or "local". Analogous to
///                   |:setglobal| and |:setlocal|, respectively.
///                 • win: |window-ID|. Used for setting window local option.
///                 • buf: Buffer number. Used for setting buffer local option.
///
///
///==============================================================================
async fn set_option_value_wv(&self, name: String, value: Object, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_set_option_value".into(), (name, value, opts, )).await
}
///nvim_get_all_options_info()                      *nvim_get_all_options_info()*
///    Gets the option information for all options.
///
///    The dict has the full option names as keys and option metadata dicts as
///    detailed at |nvim_get_option_info2()|.
///
///    Attributes: ~
///        Since: 0.5.0
///
///    Return: ~
///        dict of all options
///
///    See also: ~
///      • |nvim_get_commands()|
///
async fn get_all_options_info<D: Deserialize<'static>>(&self, ) -> error::Result<D> {
	self.call_fn("nvim_get_all_options_info".into(), [();0]).await
}
///nvim_get_all_options_info()                      *nvim_get_all_options_info()*
///    Gets the option information for all options.
///
///    The dict has the full option names as keys and option metadata dicts as
///    detailed at |nvim_get_option_info2()|.
///
///    Attributes: ~
///        Since: 0.5.0
///
///    Return: ~
///        dict of all options
///
///    See also: ~
///      • |nvim_get_commands()|
///
async fn get_all_options_info_wv(&self, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_get_all_options_info".into(), [();0]).await
}
///nvim_get_option_info2({name}, {opts})                *nvim_get_option_info2()*
///    Gets the option information for one option from arbitrary buffer or window
///
///    Resulting dict has keys:
///    • name: Name of the option (like 'filetype')
///    • shortname: Shortened name of the option (like 'ft')
///    • type: type of option ("string", "number" or "boolean")
///    • default: The default value for the option
///    • was_set: Whether the option was set.
///    • last_set_sid: Last set script id (if any)
///    • last_set_linenr: line number where option was set
///    • last_set_chan: Channel where option was set (0 for local)
///    • scope: one of "global", "win", or "buf"
///    • global_local: whether win or buf option has a global value
///    • commalist: List of comma separated values
///    • flaglist: List of single char flags
///
///    When {scope} is not provided, the last set information applies to the
///    local value in the current buffer or window if it is available, otherwise
///    the global value information is returned. This behavior can be disabled by
///    explicitly specifying {scope} in the {opts} table.
///
///    Attributes: ~
///        Since: 0.9.0
///
///    Parameters: ~
///      • {name}  Option name
///      • {opts}  Optional parameters
///                • scope: One of "global" or "local". Analogous to |:setglobal|
///                  and |:setlocal|, respectively.
///                • win: |window-ID|. Used for getting window local options.
///                • buf: Buffer number. Used for getting buffer local options.
///                  Implies {scope} is "local".
///
///    Return: ~
///        Option Information
///
async fn get_option_info2<D: Deserialize<'static>>(&self, name: &str, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_get_option_info2".into(), (name, opts, )).await
}
///nvim_get_option_info2({name}, {opts})                *nvim_get_option_info2()*
///    Gets the option information for one option from arbitrary buffer or window
///
///    Resulting dict has keys:
///    • name: Name of the option (like 'filetype')
///    • shortname: Shortened name of the option (like 'ft')
///    • type: type of option ("string", "number" or "boolean")
///    • default: The default value for the option
///    • was_set: Whether the option was set.
///    • last_set_sid: Last set script id (if any)
///    • last_set_linenr: line number where option was set
///    • last_set_chan: Channel where option was set (0 for local)
///    • scope: one of "global", "win", or "buf"
///    • global_local: whether win or buf option has a global value
///    • commalist: List of comma separated values
///    • flaglist: List of single char flags
///
///    When {scope} is not provided, the last set information applies to the
///    local value in the current buffer or window if it is available, otherwise
///    the global value information is returned. This behavior can be disabled by
///    explicitly specifying {scope} in the {opts} table.
///
///    Attributes: ~
///        Since: 0.9.0
///
///    Parameters: ~
///      • {name}  Option name
///      • {opts}  Optional parameters
///                • scope: One of "global" or "local". Analogous to |:setglobal|
///                  and |:setlocal|, respectively.
///                • win: |window-ID|. Used for getting window local options.
///                • buf: Buffer number. Used for getting buffer local options.
///                  Implies {scope} is "local".
///
///    Return: ~
///        Option Information
///
async fn get_option_info2_wv(&self, name: String, opts: Dict, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_get_option_info2".into(), (name, opts, )).await
}
///nvim_tabpage_list_wins({tabpage})                   *nvim_tabpage_list_wins()*
///    Gets the windows in a tabpage
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {tabpage}  |tab-ID|, or 0 for current tabpage
///
///    Return: ~
///        List of windows in `tabpage`
///
async fn tabpage_list_wins(&self, tabpage: Tabpage, ) -> error::Result<Vec<Window>> {
	self.call_fn("nvim_tabpage_list_wins".into(), (tabpage, )).await
}
///nvim_tabpage_get_var({tabpage}, {name})               *nvim_tabpage_get_var()*
///    Gets a tab-scoped (t:) variable
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {tabpage}  |tab-ID|, or 0 for current tabpage
///      • {name}     Variable name
///
///    Return: ~
///        Variable value
///
async fn tabpage_get_var<D: Deserialize<'static>>(&self, tabpage: Tabpage, name: &str, ) -> error::Result<D> {
	self.call_fn("nvim_tabpage_get_var".into(), (tabpage, name, )).await
}
///nvim_tabpage_get_var({tabpage}, {name})               *nvim_tabpage_get_var()*
///    Gets a tab-scoped (t:) variable
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {tabpage}  |tab-ID|, or 0 for current tabpage
///      • {name}     Variable name
///
///    Return: ~
///        Variable value
///
async fn tabpage_get_var_wv(&self, tabpage: Tabpage, name: String, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_tabpage_get_var".into(), (tabpage, name, )).await
}
///                                                      *nvim_tabpage_set_var()*
///nvim_tabpage_set_var({tabpage}, {name}, {value})
///    Sets a tab-scoped (t:) variable
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {tabpage}  |tab-ID|, or 0 for current tabpage
///      • {name}     Variable name
///      • {value}    Variable value
///
async fn tabpage_set_var(&self, tabpage: Tabpage, name: &str, value: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_tabpage_set_var".into(), (tabpage, name, value, )).await
}
///                                                      *nvim_tabpage_set_var()*
///nvim_tabpage_set_var({tabpage}, {name}, {value})
///    Sets a tab-scoped (t:) variable
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {tabpage}  |tab-ID|, or 0 for current tabpage
///      • {name}     Variable name
///      • {value}    Variable value
///
async fn tabpage_set_var_wv(&self, tabpage: Tabpage, name: String, value: Object, ) -> error::Result<()> {
	self.call_fn_wv("nvim_tabpage_set_var".into(), (tabpage, name, value, )).await
}
///nvim_tabpage_del_var({tabpage}, {name})               *nvim_tabpage_del_var()*
///    Removes a tab-scoped (t:) variable
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {tabpage}  |tab-ID|, or 0 for current tabpage
///      • {name}     Variable name
///
async fn tabpage_del_var(&self, tabpage: Tabpage, name: &str, ) -> error::Result<()> {
	self.call_fn("nvim_tabpage_del_var".into(), (tabpage, name, )).await
}
///nvim_tabpage_get_win({tabpage})                       *nvim_tabpage_get_win()*
///    Gets the current window in a tabpage
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {tabpage}  |tab-ID|, or 0 for current tabpage
///
///    Return: ~
///        |window-ID|
///
async fn tabpage_get_win(&self, tabpage: Tabpage, ) -> error::Result<Window> {
	self.call_fn("nvim_tabpage_get_win".into(), (tabpage, )).await
}
///nvim_tabpage_set_win({tabpage}, {win})                *nvim_tabpage_set_win()*
///    Sets the current window in a tabpage
///
///    Attributes: ~
///        Since: 0.10.0
///
///    Parameters: ~
///      • {tabpage}  |tab-ID|, or 0 for current tabpage
///      • {win}      |window-ID|, must already belong to {tabpage}
///
///
///==============================================================================
async fn tabpage_set_win(&self, tabpage: Tabpage, win: Window, ) -> error::Result<()> {
	self.call_fn("nvim_tabpage_set_win".into(), (tabpage, win, )).await
}
///nvim_tabpage_get_number({tabpage})                 *nvim_tabpage_get_number()*
///    Gets the tabpage number
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {tabpage}  |tab-ID|, or 0 for current tabpage
///
///    Return: ~
///        Tabpage number
///
async fn tabpage_get_number(&self, tabpage: Tabpage, ) -> error::Result<Integer> {
	self.call_fn("nvim_tabpage_get_number".into(), (tabpage, )).await
}
///nvim_tabpage_is_valid({tabpage})                     *nvim_tabpage_is_valid()*
///    Checks if a tabpage is valid
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {tabpage}  |tab-ID|, or 0 for current tabpage
///
///    Return: ~
///        true if the tabpage is valid, false otherwise
///
async fn tabpage_is_valid(&self, tabpage: Tabpage, ) -> error::Result<Boolean> {
	self.call_fn("nvim_tabpage_is_valid".into(), (tabpage, )).await
}
///nvim_ui_attach({width}, {height}, {options})                *nvim_ui_attach()*
///    Activates UI events on the channel.
///
///    Entry point of all UI clients. Allows |--embed| to continue startup.
///    Implies that the client is ready to show the UI. Adds the client to the
///    list of UIs. |nvim_list_uis()|
///
///    Note: ~
///      • If multiple UI clients are attached, the global screen dimensions
///        degrade to the smallest client. E.g. if client A requests 80x40 but
///        client B requests 200x100, the global screen has size 80x40.
///
///    Attributes: ~
///        |RPC| only
///        Since: 0.1.0
///
///    Parameters: ~
///      • {width}    Requested screen columns
///      • {height}   Requested screen rows
///      • {options}  |ui-option| map
///
async fn ui_attach(&self, width: Integer, height: Integer, options: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_ui_attach".into(), (width, height, options, )).await
}
///nvim_ui_attach({width}, {height}, {options})                *nvim_ui_attach()*
///    Activates UI events on the channel.
///
///    Entry point of all UI clients. Allows |--embed| to continue startup.
///    Implies that the client is ready to show the UI. Adds the client to the
///    list of UIs. |nvim_list_uis()|
///
///    Note: ~
///      • If multiple UI clients are attached, the global screen dimensions
///        degrade to the smallest client. E.g. if client A requests 80x40 but
///        client B requests 200x100, the global screen has size 80x40.
///
///    Attributes: ~
///        |RPC| only
///        Since: 0.1.0
///
///    Parameters: ~
///      • {width}    Requested screen columns
///      • {height}   Requested screen rows
///      • {options}  |ui-option| map
///
async fn ui_attach_wv(&self, width: Integer, height: Integer, options: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_ui_attach".into(), (width, height, options, )).await
}
///nvim_ui_set_focus({gained})                              *nvim_ui_set_focus()*
///    Tells the nvim server if focus was gained or lost by the GUI
///
///    Attributes: ~
///        |RPC| only
///        Since: 0.9.0
///
async fn ui_set_focus(&self, gained: Boolean, ) -> error::Result<()> {
	self.call_fn("nvim_ui_set_focus".into(), (gained, )).await
}
///nvim_ui_detach()                                            *nvim_ui_detach()*
///    Deactivates UI events on the channel.
///
///    Removes the client from the list of UIs. |nvim_list_uis()|
///
///    Attributes: ~
///        |RPC| only
///        Since: 0.1.0
///
async fn ui_detach(&self, ) -> error::Result<()> {
	self.call_fn("nvim_ui_detach".into(), [();0]).await
}
///nvim_ui_try_resize({width}, {height})                   *nvim_ui_try_resize()*
///
///    Attributes: ~
///        |RPC| only
///        Since: 0.1.0
///
async fn ui_try_resize(&self, width: Integer, height: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_ui_try_resize".into(), (width, height, )).await
}
///nvim_ui_set_option({name}, {value})                     *nvim_ui_set_option()*
///
///    Attributes: ~
///        |RPC| only
///        Since: 0.1.0
///
async fn ui_set_option(&self, name: &str, value: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_ui_set_option".into(), (name, value, )).await
}
///nvim_ui_set_option({name}, {value})                     *nvim_ui_set_option()*
///
///    Attributes: ~
///        |RPC| only
///        Since: 0.1.0
///
async fn ui_set_option_wv(&self, name: String, value: Object, ) -> error::Result<()> {
	self.call_fn_wv("nvim_ui_set_option".into(), (name, value, )).await
}
///                                                   *nvim_ui_try_resize_grid()*
///nvim_ui_try_resize_grid({grid}, {width}, {height})
///    Tell Nvim to resize a grid. Triggers a grid_resize event with the
///    requested grid size or the maximum size if it exceeds size limits.
///
///    On invalid grid handle, fails with error.
///
///    Attributes: ~
///        |RPC| only
///        Since: 0.4.0
///
///    Parameters: ~
///      • {grid}    The handle of the grid to be changed.
///      • {width}   The new requested width.
///      • {height}  The new requested height.
///
///
/// vim:tw=78:ts=8:sw=4:sts=4:et:ft=help:norl:
///
async fn ui_try_resize_grid(&self, grid: Integer, width: Integer, height: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_ui_try_resize_grid".into(), (grid, width, height, )).await
}
///nvim_ui_pum_set_height({height})                    *nvim_ui_pum_set_height()*
///    Tells Nvim the number of elements displaying in the popupmenu, to decide
///    <PageUp> and <PageDown> movement.
///
///    Attributes: ~
///        |RPC| only
///        Since: 0.4.0
///
///    Parameters: ~
///      • {height}  Popupmenu height, must be greater than zero.
///
async fn ui_pum_set_height(&self, height: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_ui_pum_set_height".into(), (height, )).await
}
///                                                    *nvim_ui_pum_set_bounds()*
///nvim_ui_pum_set_bounds({width}, {height}, {row}, {col})
///    Tells Nvim the geometry of the popupmenu, to align floating windows with
///    an external popup menu.
///
///    Note that this method is not to be confused with
///    |nvim_ui_pum_set_height()|, which sets the number of visible items in the
///    popup menu, while this function sets the bounding box of the popup menu,
///    including visual elements such as borders and sliders. Floats need not use
///    the same font size, nor be anchored to exact grid corners, so one can set
///    floating-point numbers to the popup menu geometry.
///
///    Attributes: ~
///        |RPC| only
///        Since: 0.5.0
///
///    Parameters: ~
///      • {width}   Popupmenu width.
///      • {height}  Popupmenu height.
///      • {row}     Popupmenu row.
///      • {col}     Popupmenu height.
///
async fn ui_pum_set_bounds(&self, width: Float, height: Float, row: Float, col: Float, ) -> error::Result<()> {
	self.call_fn("nvim_ui_pum_set_bounds".into(), (width, height, row, col, )).await
}
///nvim_ui_term_event({event}, {value})                    *nvim_ui_term_event()*
///    Tells Nvim when a terminal event has occurred
///
///    The following terminal events are supported:
///    • "termresponse": The terminal sent an OSC or DCS response sequence to
///      Nvim. The payload is the received response. Sets |v:termresponse| and
///      fires |TermResponse|.
///
///    Attributes: ~
///        |RPC| only
///        Since: 0.10.0
///
///    Parameters: ~
///      • {event}  Event name
///      • {value}  Event payload
///
async fn ui_term_event(&self, event: &str, value: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_ui_term_event".into(), (event, value, )).await
}
///nvim_ui_term_event({event}, {value})                    *nvim_ui_term_event()*
///    Tells Nvim when a terminal event has occurred
///
///    The following terminal events are supported:
///    • "termresponse": The terminal sent an OSC or DCS response sequence to
///      Nvim. The payload is the received response. Sets |v:termresponse| and
///      fires |TermResponse|.
///
///    Attributes: ~
///        |RPC| only
///        Since: 0.10.0
///
///    Parameters: ~
///      • {event}  Event name
///      • {value}  Event payload
///
async fn ui_term_event_wv(&self, event: String, value: Object, ) -> error::Result<()> {
	self.call_fn_wv("nvim_ui_term_event".into(), (event, value, )).await
}
///nvim_get_hl_id_by_name({name})                      *nvim_get_hl_id_by_name()*
///    Gets a highlight group by name
///
///    similar to |hlID()|, but allocates a new ID if not present.
///
///    Attributes: ~
///        Since: 0.5.0
///
async fn get_hl_id_by_name(&self, name: &str, ) -> error::Result<Integer> {
	self.call_fn("nvim_get_hl_id_by_name".into(), (name, )).await
}
///nvim_get_hl({ns_id}, {opts})                                   *nvim_get_hl()*
///    Gets all or specific highlight groups in a namespace.
///
///    Note: ~
///      • When the `link` attribute is defined in the highlight definition map,
///        other attributes will not be taking effect (see |:hi-link|).
///
///    Attributes: ~
///        Since: 0.9.0
///
///    Parameters: ~
///      • {ns_id}  Get highlight groups for namespace ns_id
///                 |nvim_get_namespaces()|. Use 0 to get global highlight groups
///                 |:highlight|.
///      • {opts}   Options dict:
///                 • name: (string) Get a highlight definition by name.
///                 • id: (integer) Get a highlight definition by id.
///                 • link: (boolean, default true) Show linked group name
///                   instead of effective definition |:hi-link|.
///                 • create: (boolean, default true) When highlight group
///                   doesn't exist create it.
///
///    Return: ~
///        Highlight groups as a map from group name to a highlight definition
///        map as in |nvim_set_hl()|, or only a single highlight definition map
///        if requested by name or id.
///
async fn get_hl<D: Deserialize<'static>>(&self, ns_id: Integer, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_get_hl".into(), (ns_id, opts, )).await
}
///nvim_get_hl({ns_id}, {opts})                                   *nvim_get_hl()*
///    Gets all or specific highlight groups in a namespace.
///
///    Note: ~
///      • When the `link` attribute is defined in the highlight definition map,
///        other attributes will not be taking effect (see |:hi-link|).
///
///    Attributes: ~
///        Since: 0.9.0
///
///    Parameters: ~
///      • {ns_id}  Get highlight groups for namespace ns_id
///                 |nvim_get_namespaces()|. Use 0 to get global highlight groups
///                 |:highlight|.
///      • {opts}   Options dict:
///                 • name: (string) Get a highlight definition by name.
///                 • id: (integer) Get a highlight definition by id.
///                 • link: (boolean, default true) Show linked group name
///                   instead of effective definition |:hi-link|.
///                 • create: (boolean, default true) When highlight group
///                   doesn't exist create it.
///
///    Return: ~
///        Highlight groups as a map from group name to a highlight definition
///        map as in |nvim_set_hl()|, or only a single highlight definition map
///        if requested by name or id.
///
async fn get_hl_wv(&self, ns_id: Integer, opts: Dict, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_get_hl".into(), (ns_id, opts, )).await
}
///nvim_set_hl({ns_id}, {name}, {val})                            *nvim_set_hl()*
///    Sets a highlight group.
///
///    Note: ~
///      • Unlike the `:highlight` command which can update a highlight group,
///        this function completely replaces the definition. For example:
///        `nvim_set_hl(0, 'Visual', {})` will clear the highlight group
///        'Visual'.
///      • The fg and bg keys also accept the string values `"fg"` or `"bg"`
///        which act as aliases to the corresponding foreground and background
///        values of the Normal group. If the Normal group has not been defined,
///        using these values results in an error.
///      • If `link` is used in combination with other attributes; only the
///        `link` will take effect (see |:hi-link|).
///
///    Attributes: ~
///        Since: 0.5.0
///
///    Parameters: ~
///      • {ns_id}  Namespace id for this highlight |nvim_create_namespace()|.
///                 Use 0 to set a highlight group globally |:highlight|.
///                 Highlights from non-global namespaces are not active by
///                 default, use |nvim_set_hl_ns()| or |nvim_win_set_hl_ns()| to
///                 activate them.
///      • {name}   Highlight group name, e.g. "ErrorMsg"
///      • {val}    Highlight definition map, accepts the following keys:
///                 • fg: color name or "#RRGGBB", see note.
///                 • bg: color name or "#RRGGBB", see note.
///                 • sp: color name or "#RRGGBB"
///                 • blend: integer between 0 and 100
///                 • bold: boolean
///                 • standout: boolean
///                 • underline: boolean
///                 • undercurl: boolean
///                 • underdouble: boolean
///                 • underdotted: boolean
///                 • underdashed: boolean
///                 • strikethrough: boolean
///                 • italic: boolean
///                 • reverse: boolean
///                 • nocombine: boolean
///                 • link: name of another highlight group to link to, see
///                   |:hi-link|.
///                 • default: Don't override existing definition |:hi-default|
///                 • ctermfg: Sets foreground of cterm color |ctermfg|
///                 • ctermbg: Sets background of cterm color |ctermbg|
///                 • cterm: cterm attribute map, like |highlight-args|. If not
///                   set, cterm attributes will match those from the attribute
///                   map documented above.
///                 • force: if true force update the highlight group when it
///                   exists.
///
async fn set_hl(&self, ns_id: Integer, name: &str, val: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_set_hl".into(), (ns_id, name, val, )).await
}
///nvim_set_hl({ns_id}, {name}, {val})                            *nvim_set_hl()*
///    Sets a highlight group.
///
///    Note: ~
///      • Unlike the `:highlight` command which can update a highlight group,
///        this function completely replaces the definition. For example:
///        `nvim_set_hl(0, 'Visual', {})` will clear the highlight group
///        'Visual'.
///      • The fg and bg keys also accept the string values `"fg"` or `"bg"`
///        which act as aliases to the corresponding foreground and background
///        values of the Normal group. If the Normal group has not been defined,
///        using these values results in an error.
///      • If `link` is used in combination with other attributes; only the
///        `link` will take effect (see |:hi-link|).
///
///    Attributes: ~
///        Since: 0.5.0
///
///    Parameters: ~
///      • {ns_id}  Namespace id for this highlight |nvim_create_namespace()|.
///                 Use 0 to set a highlight group globally |:highlight|.
///                 Highlights from non-global namespaces are not active by
///                 default, use |nvim_set_hl_ns()| or |nvim_win_set_hl_ns()| to
///                 activate them.
///      • {name}   Highlight group name, e.g. "ErrorMsg"
///      • {val}    Highlight definition map, accepts the following keys:
///                 • fg: color name or "#RRGGBB", see note.
///                 • bg: color name or "#RRGGBB", see note.
///                 • sp: color name or "#RRGGBB"
///                 • blend: integer between 0 and 100
///                 • bold: boolean
///                 • standout: boolean
///                 • underline: boolean
///                 • undercurl: boolean
///                 • underdouble: boolean
///                 • underdotted: boolean
///                 • underdashed: boolean
///                 • strikethrough: boolean
///                 • italic: boolean
///                 • reverse: boolean
///                 • nocombine: boolean
///                 • link: name of another highlight group to link to, see
///                   |:hi-link|.
///                 • default: Don't override existing definition |:hi-default|
///                 • ctermfg: Sets foreground of cterm color |ctermfg|
///                 • ctermbg: Sets background of cterm color |ctermbg|
///                 • cterm: cterm attribute map, like |highlight-args|. If not
///                   set, cterm attributes will match those from the attribute
///                   map documented above.
///                 • force: if true force update the highlight group when it
///                   exists.
///
async fn set_hl_wv(&self, ns_id: Integer, name: String, val: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_set_hl".into(), (ns_id, name, val, )).await
}
///nvim_get_hl_ns({opts})                                      *nvim_get_hl_ns()*
///    Gets the active highlight namespace.
///
///    Attributes: ~
///        Since: 0.10.0
///
///    Parameters: ~
///      • {opts}  Optional parameters
///                • winid: (number) |window-ID| for retrieving a window's
///                  highlight namespace. A value of -1 is returned when
///                  |nvim_win_set_hl_ns()| has not been called for the window
///                  (or was called with a namespace of -1).
///
///    Return: ~
///        Namespace id, or -1
///
async fn get_hl_ns(&self, opts: impl Serialize, ) -> error::Result<Integer> {
	self.call_fn("nvim_get_hl_ns".into(), (opts, )).await
}
///nvim_get_hl_ns({opts})                                      *nvim_get_hl_ns()*
///    Gets the active highlight namespace.
///
///    Attributes: ~
///        Since: 0.10.0
///
///    Parameters: ~
///      • {opts}  Optional parameters
///                • winid: (number) |window-ID| for retrieving a window's
///                  highlight namespace. A value of -1 is returned when
///                  |nvim_win_set_hl_ns()| has not been called for the window
///                  (or was called with a namespace of -1).
///
///    Return: ~
///        Namespace id, or -1
///
async fn get_hl_ns_wv(&self, opts: Dict, ) -> error::Result<Integer> {
	self.call_fn_wv("nvim_get_hl_ns".into(), (opts, )).await
}
///nvim_set_hl_ns({ns_id})                                     *nvim_set_hl_ns()*
///    Set active namespace for highlights defined with |nvim_set_hl()|. This can
///    be set for a single window, see |nvim_win_set_hl_ns()|.
///
///    Attributes: ~
///        Since: 0.8.0
///
///    Parameters: ~
///      • {ns_id}  the namespace to use
///
async fn set_hl_ns(&self, ns_id: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_set_hl_ns".into(), (ns_id, )).await
}
///nvim_set_hl_ns_fast({ns_id})                           *nvim_set_hl_ns_fast()*
///    Set active namespace for highlights defined with |nvim_set_hl()| while
///    redrawing.
///
///    This function meant to be called while redrawing, primarily from
///    |nvim_set_decoration_provider()| on_win and on_line callbacks, which are
///    allowed to change the namespace during a redraw cycle.
///
///    Attributes: ~
///        |api-fast|
///        Since: 0.8.0
///
///    Parameters: ~
///      • {ns_id}  the namespace to activate
///
async fn set_hl_ns_fast(&self, ns_id: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_set_hl_ns_fast".into(), (ns_id, )).await
}
///nvim_feedkeys({keys}, {mode}, {escape_ks})                   *nvim_feedkeys()*
///    Sends input-keys to Nvim, subject to various quirks controlled by `mode`
///    flags. This is a blocking call, unlike |nvim_input()|.
///
///    On execution error: does not fail, but updates v:errmsg.
///
///    To input sequences like <C-o> use |nvim_replace_termcodes()| (typically
///    with escape_ks=false) to replace |keycodes|, then pass the result to
///    nvim_feedkeys().
///
///    Example: >vim
///        :let key = nvim_replace_termcodes("<C-o>", v:true, v:false, v:true)
///        :call nvim_feedkeys(key, 'n', v:false)
///<
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {keys}       to be typed
///      • {mode}       behavior flags, see |feedkeys()|
///      • {escape_ks}  If true, escape K_SPECIAL bytes in `keys`. This should be
///                     false if you already used |nvim_replace_termcodes()|, and
///                     true otherwise.
///
///    See also: ~
///      • feedkeys()
///      • vim_strsave_escape_ks
///
async fn feedkeys(&self, keys: &str, mode: &str, escape_ks: Boolean, ) -> error::Result<()> {
	self.call_fn("nvim_feedkeys".into(), (keys, mode, escape_ks, )).await
}
///nvim_input({keys})                                              *nvim_input()*
///    Queues raw user-input. Unlike |nvim_feedkeys()|, this uses a low-level
///    input buffer and the call is non-blocking (input is processed
///    asynchronously by the eventloop).
///
///    To input blocks of text, |nvim_paste()| is much faster and should be
///    preferred.
///
///    On execution error: does not fail, but updates v:errmsg.
///
///    Note: ~
///      • |keycodes| like <CR> are translated, so "<" is special. To input a
///        literal "<", send <LT>.
///      • For mouse events use |nvim_input_mouse()|. The pseudokey form
///        `<LeftMouse><col,row>` is deprecated since |api-level| 6.
///
///    Attributes: ~
///        |api-fast|
///        Since: 0.1.0
///
///    Parameters: ~
///      • {keys}  to be typed
///
///    Return: ~
///        Number of bytes actually written (can be fewer than requested if the
///        buffer becomes full).
///
async fn input(&self, keys: &str, ) -> error::Result<Integer> {
	self.call_fn("nvim_input".into(), (keys, )).await
}
///                                                          *nvim_input_mouse()*
///nvim_input_mouse({button}, {action}, {modifier}, {grid}, {row}, {col})
///    Send mouse event from GUI.
///
///    Non-blocking: does not wait on any result, but queues the event to be
///    processed soon by the event loop.
///
///    Note: ~
///      • Currently this doesn't support "scripting" multiple mouse events by
///        calling it multiple times in a loop: the intermediate mouse positions
///        will be ignored. It should be used to implement real-time mouse input
///        in a GUI. The deprecated pseudokey form (`<LeftMouse><col,row>`) of
///        |nvim_input()| has the same limitation.
///
///    Attributes: ~
///        |api-fast|
///        Since: 0.4.0
///
///    Parameters: ~
///      • {button}    Mouse button: one of "left", "right", "middle", "wheel",
///                    "move", "x1", "x2".
///      • {action}    For ordinary buttons, one of "press", "drag", "release".
///                    For the wheel, one of "up", "down", "left", "right".
///                    Ignored for "move".
///      • {modifier}  String of modifiers each represented by a single char. The
///                    same specifiers are used as for a key press, except that
///                    the "-" separator is optional, so "C-A-", "c-a" and "CA"
///                    can all be used to specify Ctrl+Alt+click.
///      • {grid}      Grid number if the client uses |ui-multigrid|, else 0.
///      • {row}       Mouse row-position (zero-based, like redraw events)
///      • {col}       Mouse column-position (zero-based, like redraw events)
///
async fn input_mouse(&self, button: &str, action: &str, modifier: &str, grid: Integer, row: Integer, col: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_input_mouse".into(), (button, action, modifier, grid, row, col, )).await
}
///                                                    *nvim_replace_termcodes()*
///nvim_replace_termcodes({str}, {from_part}, {do_lt}, {special})
///    Replaces terminal codes and |keycodes| (<CR>, <Esc>, ...) in a string with
///    the internal representation.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {str}        String to be converted.
///      • {from_part}  Legacy Vim parameter. Usually true.
///      • {do_lt}      Also translate <lt>. Ignored if `special` is false.
///      • {special}    Replace |keycodes|, e.g. <CR> becomes a "\r" char.
///
///    See also: ~
///      • replace_termcodes
///      • cpoptions
///
async fn replace_termcodes(&self, str: &str, from_part: Boolean, do_lt: Boolean, special: Boolean, ) -> error::Result<String> {
	self.call_fn("nvim_replace_termcodes".into(), (str, from_part, do_lt, special, )).await
}
///nvim_exec_lua({code}, {args})                                *nvim_exec_lua()*
///    Execute Lua code. Parameters (if any) are available as `...` inside the
///    chunk. The chunk can return a value.
///
///    Only statements are executed. To evaluate an expression, prefix it with
///    `return`: return my_function(...)
///
///    Attributes: ~
///        |RPC| only
///        Since: 0.5.0
///
///    Parameters: ~
///      • {code}  Lua code to execute
///      • {args}  Arguments to the code
///
///    Return: ~
///        Return value of Lua code if present or NIL.
///
async fn exec_lua<D: Deserialize<'static>>(&self, code: &str, args: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_exec_lua".into(), (code, args, )).await
}
///nvim_exec_lua({code}, {args})                                *nvim_exec_lua()*
///    Execute Lua code. Parameters (if any) are available as `...` inside the
///    chunk. The chunk can return a value.
///
///    Only statements are executed. To evaluate an expression, prefix it with
///    `return`: return my_function(...)
///
///    Attributes: ~
///        |RPC| only
///        Since: 0.5.0
///
///    Parameters: ~
///      • {code}  Lua code to execute
///      • {args}  Arguments to the code
///
///    Return: ~
///        Return value of Lua code if present or NIL.
///
async fn exec_lua_wv(&self, code: String, args: Array, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_exec_lua".into(), (code, args, )).await
}
///nvim_strwidth({text})                                        *nvim_strwidth()*
///    Calculates the number of display cells occupied by `text`. Control
///    characters including <Tab> count as one cell.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {text}  Some text
///
///    Return: ~
///        Number of cells
///
async fn strwidth(&self, text: &str, ) -> error::Result<Integer> {
	self.call_fn("nvim_strwidth".into(), (text, )).await
}
///nvim_list_runtime_paths()                          *nvim_list_runtime_paths()*
///    Gets the paths contained in |runtime-search-path|.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Return: ~
///        List of paths
///
async fn list_runtime_paths(&self, ) -> error::Result<Vec<String>> {
	self.call_fn("nvim_list_runtime_paths".into(), [();0]).await
}
///nvim_get_runtime_file({name}, {all})                 *nvim_get_runtime_file()*
///    Finds files in runtime directories, in 'runtimepath' order.
///
///    "name" can contain wildcards. For example
///    `nvim_get_runtime_file("colors/*.{vim,lua}", true)` will return all color
///    scheme files. Always use forward slashes (/) in the search pattern for
///    subdirectories regardless of platform.
///
///    It is not an error to not find any files. An empty array is returned then.
///
///    Attributes: ~
///        |api-fast|
///        Since: 0.5.0
///
///    Parameters: ~
///      • {name}  pattern of files to search for
///      • {all}   whether to return all matches or only the first
///
///    Return: ~
///        list of absolute paths to the found files
///
async fn get_runtime_file(&self, name: &str, all: Boolean, ) -> error::Result<Vec<String>> {
	self.call_fn("nvim_get_runtime_file".into(), (name, all, )).await
}
///nvim_set_current_dir({dir})                           *nvim_set_current_dir()*
///    Changes the global working directory.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {dir}  Directory path
///
async fn set_current_dir(&self, dir: &str, ) -> error::Result<()> {
	self.call_fn("nvim_set_current_dir".into(), (dir, )).await
}
///nvim_get_current_line()                              *nvim_get_current_line()*
///    Gets the current line.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Return: ~
///        Current line string
///
async fn get_current_line(&self, ) -> error::Result<String> {
	self.call_fn("nvim_get_current_line".into(), [();0]).await
}
///nvim_set_current_line({line})                        *nvim_set_current_line()*
///    Sets the text on the current line.
///
///    Attributes: ~
///        not allowed when |textlock| is active
///        Since: 0.1.0
///
///    Parameters: ~
///      • {line}  Line contents
///
async fn set_current_line(&self, line: &str, ) -> error::Result<()> {
	self.call_fn("nvim_set_current_line".into(), (line, )).await
}
///nvim_del_current_line()                              *nvim_del_current_line()*
///    Deletes the current line.
///
///    Attributes: ~
///        not allowed when |textlock| is active
///        Since: 0.1.0
///
async fn del_current_line(&self, ) -> error::Result<()> {
	self.call_fn("nvim_del_current_line".into(), [();0]).await
}
///nvim_get_var({name})                                          *nvim_get_var()*
///    Gets a global (g:) variable.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {name}  Variable name
///
///    Return: ~
///        Variable value
///
async fn get_var<D: Deserialize<'static>>(&self, name: &str, ) -> error::Result<D> {
	self.call_fn("nvim_get_var".into(), (name, )).await
}
///nvim_get_var({name})                                          *nvim_get_var()*
///    Gets a global (g:) variable.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {name}  Variable name
///
///    Return: ~
///        Variable value
///
async fn get_var_wv(&self, name: String, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_get_var".into(), (name, )).await
}
///nvim_set_var({name}, {value})                                 *nvim_set_var()*
///    Sets a global (g:) variable.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {name}   Variable name
///      • {value}  Variable value
///
async fn set_var(&self, name: &str, value: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_set_var".into(), (name, value, )).await
}
///nvim_set_var({name}, {value})                                 *nvim_set_var()*
///    Sets a global (g:) variable.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {name}   Variable name
///      • {value}  Variable value
///
async fn set_var_wv(&self, name: String, value: Object, ) -> error::Result<()> {
	self.call_fn_wv("nvim_set_var".into(), (name, value, )).await
}
///nvim_del_var({name})                                          *nvim_del_var()*
///    Removes a global (g:) variable.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {name}  Variable name
///
async fn del_var(&self, name: &str, ) -> error::Result<()> {
	self.call_fn("nvim_del_var".into(), (name, )).await
}
///nvim_get_vvar({name})                                        *nvim_get_vvar()*
///    Gets a v: variable.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {name}  Variable name
///
///    Return: ~
///        Variable value
///
async fn get_vvar<D: Deserialize<'static>>(&self, name: &str, ) -> error::Result<D> {
	self.call_fn("nvim_get_vvar".into(), (name, )).await
}
///nvim_get_vvar({name})                                        *nvim_get_vvar()*
///    Gets a v: variable.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {name}  Variable name
///
///    Return: ~
///        Variable value
///
async fn get_vvar_wv(&self, name: String, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_get_vvar".into(), (name, )).await
}
///nvim_set_vvar({name}, {value})                               *nvim_set_vvar()*
///    Sets a v: variable, if it is not readonly.
///
///    Attributes: ~
///        Since: 0.4.0
///
///    Parameters: ~
///      • {name}   Variable name
///      • {value}  Variable value
///
async fn set_vvar(&self, name: &str, value: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_set_vvar".into(), (name, value, )).await
}
///nvim_set_vvar({name}, {value})                               *nvim_set_vvar()*
///    Sets a v: variable, if it is not readonly.
///
///    Attributes: ~
///        Since: 0.4.0
///
///    Parameters: ~
///      • {name}   Variable name
///      • {value}  Variable value
///
async fn set_vvar_wv(&self, name: String, value: Object, ) -> error::Result<()> {
	self.call_fn_wv("nvim_set_vvar".into(), (name, value, )).await
}
///nvim_echo({chunks}, {history}, {opts})                           *nvim_echo()*
///    Prints a message given by a list of `[text, hl_group]` "chunks".
///
///    Example: >lua
///        vim.api.nvim_echo({ { 'chunk1-line1\nchunk1-line2\n' }, { 'chunk2-line1' } }, true, {})
///<
///
///    Attributes: ~
///        Since: 0.5.0
///
///    Parameters: ~
///      • {chunks}   List of `[text, hl_group]` pairs, where each is a `text`
///                   string highlighted by the (optional) name or ID `hl_group`.
///      • {history}  if true, add to |message-history|.
///      • {opts}     Optional parameters.
///                   • err: Treat the message like `:echoerr`. Sets `hl_group`
///                     to |hl-ErrorMsg| by default.
///                   • verbose: Message is controlled by the 'verbose' option.
///                     Nvim invoked with `-V3log` will write the message to the
///                     "log" file instead of standard output.
///
async fn echo(&self, chunks: impl Serialize, history: Boolean, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_echo".into(), (chunks, history, opts, )).await
}
///nvim_echo({chunks}, {history}, {opts})                           *nvim_echo()*
///    Prints a message given by a list of `[text, hl_group]` "chunks".
///
///    Example: >lua
///        vim.api.nvim_echo({ { 'chunk1-line1\nchunk1-line2\n' }, { 'chunk2-line1' } }, true, {})
///<
///
///    Attributes: ~
///        Since: 0.5.0
///
///    Parameters: ~
///      • {chunks}   List of `[text, hl_group]` pairs, where each is a `text`
///                   string highlighted by the (optional) name or ID `hl_group`.
///      • {history}  if true, add to |message-history|.
///      • {opts}     Optional parameters.
///                   • err: Treat the message like `:echoerr`. Sets `hl_group`
///                     to |hl-ErrorMsg| by default.
///                   • verbose: Message is controlled by the 'verbose' option.
///                     Nvim invoked with `-V3log` will write the message to the
///                     "log" file instead of standard output.
///
async fn echo_wv(&self, chunks: Array, history: Boolean, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_echo".into(), (chunks, history, opts, )).await
}
///nvim_list_bufs()                                            *nvim_list_bufs()*
///    Gets the current list of buffers.
///
///    Includes unlisted (unloaded/deleted) buffers, like `:ls!`. Use
///    |nvim_buf_is_loaded()| to check if a buffer is loaded.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Return: ~
///        List of buffer ids
///
async fn list_bufs(&self, ) -> error::Result<Vec<Buffer>> {
	self.call_fn("nvim_list_bufs".into(), [();0]).await
}
///nvim_get_current_buf()                                *nvim_get_current_buf()*
///    Gets the current buffer.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Return: ~
///        Buffer id
///
async fn get_current_buf(&self, ) -> error::Result<Buffer> {
	self.call_fn("nvim_get_current_buf".into(), [();0]).await
}
///nvim_set_current_buf({buffer})                        *nvim_set_current_buf()*
///    Sets the current window's buffer to `buffer`.
///
///    Attributes: ~
///        not allowed when |textlock| is active or in the |cmdwin|
///        Since: 0.1.0
///
///    Parameters: ~
///      • {buffer}  Buffer id
///
async fn set_current_buf(&self, buffer: Buffer, ) -> error::Result<()> {
	self.call_fn("nvim_set_current_buf".into(), (buffer, )).await
}
///nvim_list_wins()                                            *nvim_list_wins()*
///    Gets the current list of all |window-ID|s in all tabpages.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Return: ~
///        List of |window-ID|s
///
async fn list_wins(&self, ) -> error::Result<Vec<Window>> {
	self.call_fn("nvim_list_wins".into(), [();0]).await
}
///nvim_get_current_win()                                *nvim_get_current_win()*
///    Gets the current window.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Return: ~
///        |window-ID|
///
async fn get_current_win(&self, ) -> error::Result<Window> {
	self.call_fn("nvim_get_current_win".into(), [();0]).await
}
///nvim_set_current_win({window})                        *nvim_set_current_win()*
///    Sets the current window (and tabpage, implicitly).
///
///    Attributes: ~
///        not allowed when |textlock| is active or in the |cmdwin|
///        Since: 0.1.0
///
///    Parameters: ~
///      • {window}  |window-ID| to focus
///
async fn set_current_win(&self, window: Window, ) -> error::Result<()> {
	self.call_fn("nvim_set_current_win".into(), (window, )).await
}
///nvim_create_buf({listed}, {scratch})                       *nvim_create_buf()*
///    Creates a new, empty, unnamed buffer.
///
///    Attributes: ~
///        Since: 0.4.0
///
///    Parameters: ~
///      • {listed}   Sets 'buflisted'
///      • {scratch}  Creates a "throwaway" |scratch-buffer| for temporary work
///                   (always 'nomodified'). Also sets 'nomodeline' on the
///                   buffer.
///
///    Return: ~
///        Buffer id, or 0 on error
///
///    See also: ~
///      • buf_open_scratch
///
async fn create_buf(&self, listed: Boolean, scratch: Boolean, ) -> error::Result<Buffer> {
	self.call_fn("nvim_create_buf".into(), (listed, scratch, )).await
}
///nvim_open_term({buffer}, {opts})                            *nvim_open_term()*
///    Open a terminal instance in a buffer
///
///    By default (and currently the only option) the terminal will not be
///    connected to an external process. Instead, input sent on the channel will
///    be echoed directly by the terminal. This is useful to display ANSI
///    terminal sequences returned as part of a rpc message, or similar.
///
///    Note: to directly initiate the terminal using the right size, display the
///    buffer in a configured window before calling this. For instance, for a
///    floating display, first create an empty buffer using |nvim_create_buf()|,
///    then display it using |nvim_open_win()|, and then call this function. Then
///    |nvim_chan_send()| can be called immediately to process sequences in a
///    virtual terminal having the intended size.
///
///    Example: this `TermHl` command can be used to display and highlight raw
///    ANSI termcodes, so you can use Nvim as a "scrollback pager" (for terminals
///    like kitty):                     *ansi-colorize* *terminal-scrollback-pager* >lua
///        vim.api.nvim_create_user_command('TermHl', function()
///          local b = vim.api.nvim_create_buf(false, true)
///          local chan = vim.api.nvim_open_term(b, {})
///          vim.api.nvim_chan_send(chan, table.concat(vim.api.nvim_buf_get_lines(0, 0, -1, false), '\n'))
///          vim.api.nvim_win_set_buf(0, b)
///        end, { desc = 'Highlights ANSI termcodes in curbuf' })
///<
///
///    Attributes: ~
///        not allowed when |textlock| is active
///        Since: 0.5.0
///
///    Parameters: ~
///      • {buffer}  the buffer to use (expected to be empty)
///      • {opts}    Optional parameters.
///                  • on_input: Lua callback for input sent, i e keypresses in
///                    terminal mode. Note: keypresses are sent raw as they would
///                    be to the pty master end. For instance, a carriage return
///                    is sent as a "\r", not as a "\n". |textlock| applies. It
///                    is possible to call |nvim_chan_send()| directly in the
///                    callback however. `["input", term, bufnr, data]`
///                  • force_crlf: (boolean, default true) Convert "\n" to
///                    "\r\n".
///
///    Return: ~
///        Channel id, or 0 on error
///
async fn open_term(&self, buffer: Buffer, opts: impl Serialize, ) -> error::Result<Integer> {
	self.call_fn("nvim_open_term".into(), (buffer, opts, )).await
}
///nvim_open_term({buffer}, {opts})                            *nvim_open_term()*
///    Open a terminal instance in a buffer
///
///    By default (and currently the only option) the terminal will not be
///    connected to an external process. Instead, input sent on the channel will
///    be echoed directly by the terminal. This is useful to display ANSI
///    terminal sequences returned as part of a rpc message, or similar.
///
///    Note: to directly initiate the terminal using the right size, display the
///    buffer in a configured window before calling this. For instance, for a
///    floating display, first create an empty buffer using |nvim_create_buf()|,
///    then display it using |nvim_open_win()|, and then call this function. Then
///    |nvim_chan_send()| can be called immediately to process sequences in a
///    virtual terminal having the intended size.
///
///    Example: this `TermHl` command can be used to display and highlight raw
///    ANSI termcodes, so you can use Nvim as a "scrollback pager" (for terminals
///    like kitty):                     *ansi-colorize* *terminal-scrollback-pager* >lua
///        vim.api.nvim_create_user_command('TermHl', function()
///          local b = vim.api.nvim_create_buf(false, true)
///          local chan = vim.api.nvim_open_term(b, {})
///          vim.api.nvim_chan_send(chan, table.concat(vim.api.nvim_buf_get_lines(0, 0, -1, false), '\n'))
///          vim.api.nvim_win_set_buf(0, b)
///        end, { desc = 'Highlights ANSI termcodes in curbuf' })
///<
///
///    Attributes: ~
///        not allowed when |textlock| is active
///        Since: 0.5.0
///
///    Parameters: ~
///      • {buffer}  the buffer to use (expected to be empty)
///      • {opts}    Optional parameters.
///                  • on_input: Lua callback for input sent, i e keypresses in
///                    terminal mode. Note: keypresses are sent raw as they would
///                    be to the pty master end. For instance, a carriage return
///                    is sent as a "\r", not as a "\n". |textlock| applies. It
///                    is possible to call |nvim_chan_send()| directly in the
///                    callback however. `["input", term, bufnr, data]`
///                  • force_crlf: (boolean, default true) Convert "\n" to
///                    "\r\n".
///
///    Return: ~
///        Channel id, or 0 on error
///
async fn open_term_wv(&self, buffer: Buffer, opts: Dict, ) -> error::Result<Integer> {
	self.call_fn_wv("nvim_open_term".into(), (buffer, opts, )).await
}
///nvim_chan_send({chan}, {data})                              *nvim_chan_send()*
///    Send data to channel `id`. For a job, it writes it to the stdin of the
///    process. For the stdio channel |channel-stdio|, it writes to Nvim's
///    stdout. For an internal terminal instance (|nvim_open_term()|) it writes
///    directly to terminal output. See |channel-bytes| for more information.
///
///    This function writes raw data, not RPC messages. If the channel was
///    created with `rpc=true` then the channel expects RPC messages, use
///    |vim.rpcnotify()| and |vim.rpcrequest()| instead.
///
///    Attributes: ~
///        |RPC| only
///        Lua |vim.api| only
///        Since: 0.5.0
///
///    Parameters: ~
///      • {chan}  id of the channel
///      • {data}  data to write. 8-bit clean: can contain NUL bytes.
///
async fn chan_send(&self, chan: Integer, data: &str, ) -> error::Result<()> {
	self.call_fn("nvim_chan_send".into(), (chan, data, )).await
}
///nvim_list_tabpages()                                    *nvim_list_tabpages()*
///    Gets the current list of |tab-ID|s.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Return: ~
///        List of |tab-ID|s
///
async fn list_tabpages(&self, ) -> error::Result<Vec<Tabpage>> {
	self.call_fn("nvim_list_tabpages".into(), [();0]).await
}
///nvim_get_current_tabpage()                        *nvim_get_current_tabpage()*
///    Gets the current tabpage.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Return: ~
///        |tab-ID|
///
async fn get_current_tabpage(&self, ) -> error::Result<Tabpage> {
	self.call_fn("nvim_get_current_tabpage".into(), [();0]).await
}
///nvim_set_current_tabpage({tabpage})               *nvim_set_current_tabpage()*
///    Sets the current tabpage.
///
///    Attributes: ~
///        not allowed when |textlock| is active or in the |cmdwin|
///        Since: 0.1.0
///
///    Parameters: ~
///      • {tabpage}  |tab-ID| to focus
///
async fn set_current_tabpage(&self, tabpage: Tabpage, ) -> error::Result<()> {
	self.call_fn("nvim_set_current_tabpage".into(), (tabpage, )).await
}
///nvim_paste({data}, {crlf}, {phase})                             *nvim_paste()*
///    Pastes at cursor (in any mode), and sets "redo" so dot (|.|) will repeat
///    the input. UIs call this to implement "paste", but it's also intended for
///    use by scripts to input large, dot-repeatable blocks of text (as opposed
///    to |nvim_input()| which is subject to mappings/events and is thus much
///    slower).
///
///    Invokes the |vim.paste()| handler, which handles each mode appropriately.
///
///    Errors ('nomodifiable', `vim.paste()` failure, …) are reflected in `err`
///    but do not affect the return value (which is strictly decided by
///    `vim.paste()`). On error or cancel, subsequent calls are ignored
///    ("drained") until the next paste is initiated (phase 1 or -1).
///
///    Useful in mappings and scripts to insert multiline text. Example: >lua
///        vim.keymap.set('n', 'x', function()
///          vim.api.nvim_paste([[
///            line1
///            line2
///            line3
///          ]], false, -1)
///        end, { buffer = true })
///<
///
///    Attributes: ~
///        not allowed when |textlock| is active
///        Since: 0.4.0
///
///    Parameters: ~
///      • {data}   Multiline input. Lines break at LF ("\n"). May be binary
///                 (containing NUL bytes).
///      • {crlf}   Also break lines at CR and CRLF.
///      • {phase}  -1: paste in a single call (i.e. without streaming). To
///                 "stream" a paste, call `nvim_paste` sequentially with these
///                 `phase` values:
///                 • 1: starts the paste (exactly once)
///                 • 2: continues the paste (zero or more times)
///                 • 3: ends the paste (exactly once)
///
///    Return: ~
///        • true: Client may continue pasting.
///        • false: Client should cancel the paste.
///
async fn paste(&self, data: &str, crlf: Boolean, phase: Integer, ) -> error::Result<Boolean> {
	self.call_fn("nvim_paste".into(), (data, crlf, phase, )).await
}
///nvim_put({lines}, {type}, {after}, {follow})                      *nvim_put()*
///    Puts text at cursor, in any mode. For dot-repeatable input, use
///    |nvim_paste()|.
///
///    Compare |:put| and |p| which are always linewise.
///
///    Attributes: ~
///        not allowed when |textlock| is active
///        Since: 0.4.0
///
///    Parameters: ~
///      • {lines}   |readfile()|-style list of lines. |channel-lines|
///      • {type}    Edit behavior: any |getregtype()| result, or:
///                  • "b" |blockwise-visual| mode (may include width, e.g. "b3")
///                  • "c" |charwise| mode
///                  • "l" |linewise| mode
///                  • "" guess by contents, see |setreg()|
///      • {after}   If true insert after cursor (like |p|), or before (like
///                  |P|).
///      • {follow}  If true place cursor at end of inserted text.
///
async fn put(&self, lines: &[&str], type_: &str, after: Boolean, follow: Boolean, ) -> error::Result<()> {
	self.call_fn("nvim_put".into(), (lines, type_, after, follow, )).await
}
///nvim_get_color_by_name({name})                      *nvim_get_color_by_name()*
///    Returns the 24-bit RGB value of a |nvim_get_color_map()| color name or
///    "#rrggbb" hexadecimal string.
///
///    Example: >vim
///        :echo nvim_get_color_by_name("Pink")
///        :echo nvim_get_color_by_name("#cbcbcb")
///<
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {name}  Color name or "#rrggbb" string
///
///    Return: ~
///        24-bit RGB value, or -1 for invalid argument.
///
async fn get_color_by_name(&self, name: &str, ) -> error::Result<Integer> {
	self.call_fn("nvim_get_color_by_name".into(), (name, )).await
}
///nvim_get_color_map()                                    *nvim_get_color_map()*
///    Returns a map of color names and RGB values.
///
///    Keys are color names (e.g. "Aqua") and values are 24-bit RGB color values
///    (e.g. 65535).
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Return: ~
///        Map of color names and RGB values.
///
async fn get_color_map<D: Deserialize<'static>>(&self, ) -> error::Result<D> {
	self.call_fn("nvim_get_color_map".into(), [();0]).await
}
///nvim_get_color_map()                                    *nvim_get_color_map()*
///    Returns a map of color names and RGB values.
///
///    Keys are color names (e.g. "Aqua") and values are 24-bit RGB color values
///    (e.g. 65535).
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Return: ~
///        Map of color names and RGB values.
///
async fn get_color_map_wv(&self, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_get_color_map".into(), [();0]).await
}
///nvim_get_context({opts})                                  *nvim_get_context()*
///    Gets a map of the current editor state.
///
///    Attributes: ~
///        Since: 0.4.0
///
///    Parameters: ~
///      • {opts}  Optional parameters.
///                • types: List of |context-types| ("regs", "jumps", "bufs",
///                  "gvars", …) to gather, or empty for "all".
///
///    Return: ~
///        map of global |context|.
///
async fn get_context<D: Deserialize<'static>>(&self, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_get_context".into(), (opts, )).await
}
///nvim_get_context({opts})                                  *nvim_get_context()*
///    Gets a map of the current editor state.
///
///    Attributes: ~
///        Since: 0.4.0
///
///    Parameters: ~
///      • {opts}  Optional parameters.
///                • types: List of |context-types| ("regs", "jumps", "bufs",
///                  "gvars", …) to gather, or empty for "all".
///
///    Return: ~
///        map of global |context|.
///
async fn get_context_wv(&self, opts: Dict, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_get_context".into(), (opts, )).await
}
///nvim_load_context({dict})                                *nvim_load_context()*
///    Sets the current editor state from the given |context| map.
///
///    Attributes: ~
///        Since: 0.4.0
///
///    Parameters: ~
///      • {dict}  |Context| map.
///
async fn load_context<D: Deserialize<'static>>(&self, dict: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_load_context".into(), (dict, )).await
}
///nvim_load_context({dict})                                *nvim_load_context()*
///    Sets the current editor state from the given |context| map.
///
///    Attributes: ~
///        Since: 0.4.0
///
///    Parameters: ~
///      • {dict}  |Context| map.
///
async fn load_context_wv(&self, dict: Dict, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_load_context".into(), (dict, )).await
}
///nvim_get_mode()                                              *nvim_get_mode()*
///    Gets the current mode. |mode()| "blocking" is true if Nvim is waiting for
///    input.
///
///    Attributes: ~
///        |api-fast|
///        Since: 0.2.0
///
///    Return: ~
///        Dict { "mode": String, "blocking": Boolean }
///
async fn get_mode<D: Deserialize<'static>>(&self, ) -> error::Result<D> {
	self.call_fn("nvim_get_mode".into(), [();0]).await
}
///nvim_get_mode()                                              *nvim_get_mode()*
///    Gets the current mode. |mode()| "blocking" is true if Nvim is waiting for
///    input.
///
///    Attributes: ~
///        |api-fast|
///        Since: 0.2.0
///
///    Return: ~
///        Dict { "mode": String, "blocking": Boolean }
///
async fn get_mode_wv(&self, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_get_mode".into(), [();0]).await
}
///nvim_get_keymap({mode})                                    *nvim_get_keymap()*
///    Gets a list of global (non-buffer-local) |mapping| definitions.
///
///    Attributes: ~
///        Since: 0.2.1
///
///    Parameters: ~
///      • {mode}  Mode short-name ("n", "i", "v", ...)
///
///    Return: ~
///        Array of |maparg()|-like dictionaries describing mappings. The
///        "buffer" key is always zero.
///
async fn get_keymap<D: Deserialize<'static>>(&self, mode: &str, ) -> error::Result<D> {
	self.call_fn("nvim_get_keymap".into(), (mode, )).await
}
///nvim_get_keymap({mode})                                    *nvim_get_keymap()*
///    Gets a list of global (non-buffer-local) |mapping| definitions.
///
///    Attributes: ~
///        Since: 0.2.1
///
///    Parameters: ~
///      • {mode}  Mode short-name ("n", "i", "v", ...)
///
///    Return: ~
///        Array of |maparg()|-like dictionaries describing mappings. The
///        "buffer" key is always zero.
///
async fn get_keymap_wv(&self, mode: String, ) -> error::Result<Vec<Dict>> {
	self.call_fn_wv("nvim_get_keymap".into(), (mode, )).await
}
///nvim_set_keymap({mode}, {lhs}, {rhs}, {opts})              *nvim_set_keymap()*
///    Sets a global |mapping| for the given mode.
///
///    To set a buffer-local mapping, use |nvim_buf_set_keymap()|.
///
///    Unlike |:map|, leading/trailing whitespace is accepted as part of the
///    {lhs} or {rhs}. Empty {rhs} is <Nop>. |keycodes| are replaced as usual.
///
///    Example: >vim
///        call nvim_set_keymap('n', ' <NL>', '', {'nowait': v:true})
///<
///
///    is equivalent to: >vim
///        nmap <nowait> <Space><NL> <Nop>
///<
///
///    Attributes: ~
///        Since: 0.4.0
///
///    Parameters: ~
///      • {mode}  Mode short-name (map command prefix: "n", "i", "v", "x", …)
///                or "!" for |:map!|, or empty string for |:map|. "ia", "ca" or
///                "!a" for abbreviation in Insert mode, Cmdline mode, or both,
///                respectively
///      • {lhs}   Left-hand-side |{lhs}| of the mapping.
///      • {rhs}   Right-hand-side |{rhs}| of the mapping.
///      • {opts}  Optional parameters map: Accepts all |:map-arguments| as keys
///                except <buffer>, values are booleans (default false). Also:
///                • "noremap" disables |recursive_mapping|, like |:noremap|
///                • "desc" human-readable description.
///                • "callback" Lua function called in place of {rhs}.
///                • "replace_keycodes" (boolean) When "expr" is true, replace
///                  keycodes in the resulting string (see
///                  |nvim_replace_termcodes()|). Returning nil from the Lua
///                  "callback" is equivalent to returning an empty string.
///
async fn set_keymap(&self, mode: &str, lhs: &str, rhs: &str, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_set_keymap".into(), (mode, lhs, rhs, opts, )).await
}
///nvim_set_keymap({mode}, {lhs}, {rhs}, {opts})              *nvim_set_keymap()*
///    Sets a global |mapping| for the given mode.
///
///    To set a buffer-local mapping, use |nvim_buf_set_keymap()|.
///
///    Unlike |:map|, leading/trailing whitespace is accepted as part of the
///    {lhs} or {rhs}. Empty {rhs} is <Nop>. |keycodes| are replaced as usual.
///
///    Example: >vim
///        call nvim_set_keymap('n', ' <NL>', '', {'nowait': v:true})
///<
///
///    is equivalent to: >vim
///        nmap <nowait> <Space><NL> <Nop>
///<
///
///    Attributes: ~
///        Since: 0.4.0
///
///    Parameters: ~
///      • {mode}  Mode short-name (map command prefix: "n", "i", "v", "x", …)
///                or "!" for |:map!|, or empty string for |:map|. "ia", "ca" or
///                "!a" for abbreviation in Insert mode, Cmdline mode, or both,
///                respectively
///      • {lhs}   Left-hand-side |{lhs}| of the mapping.
///      • {rhs}   Right-hand-side |{rhs}| of the mapping.
///      • {opts}  Optional parameters map: Accepts all |:map-arguments| as keys
///                except <buffer>, values are booleans (default false). Also:
///                • "noremap" disables |recursive_mapping|, like |:noremap|
///                • "desc" human-readable description.
///                • "callback" Lua function called in place of {rhs}.
///                • "replace_keycodes" (boolean) When "expr" is true, replace
///                  keycodes in the resulting string (see
///                  |nvim_replace_termcodes()|). Returning nil from the Lua
///                  "callback" is equivalent to returning an empty string.
///
async fn set_keymap_wv(&self, mode: String, lhs: String, rhs: String, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_set_keymap".into(), (mode, lhs, rhs, opts, )).await
}
///nvim_del_keymap({mode}, {lhs})                             *nvim_del_keymap()*
///    Unmaps a global |mapping| for the given mode.
///
///    To unmap a buffer-local mapping, use |nvim_buf_del_keymap()|.
///
///    Attributes: ~
///        Since: 0.4.0
///
///    See also: ~
///      • |nvim_set_keymap()|
///
async fn del_keymap(&self, mode: &str, lhs: &str, ) -> error::Result<()> {
	self.call_fn("nvim_del_keymap".into(), (mode, lhs, )).await
}
///nvim_get_api_info()                                      *nvim_get_api_info()*
///    Returns a 2-tuple (Array), where item 0 is the current channel id and item
///    1 is the |api-metadata| map (Dict).
///
///    Attributes: ~
///        |api-fast|
///        |RPC| only
///        Since: 0.1.0
///
///    Return: ~
///        2-tuple `[{channel-id}, {api-metadata}]`
///
async fn get_api_info<D: Deserialize<'static>>(&self, ) -> error::Result<D> {
	self.call_fn("nvim_get_api_info".into(), [();0]).await
}
///nvim_get_api_info()                                      *nvim_get_api_info()*
///    Returns a 2-tuple (Array), where item 0 is the current channel id and item
///    1 is the |api-metadata| map (Dict).
///
///    Attributes: ~
///        |api-fast|
///        |RPC| only
///        Since: 0.1.0
///
///    Return: ~
///        2-tuple `[{channel-id}, {api-metadata}]`
///
async fn get_api_info_wv(&self, ) -> error::Result<Array> {
	self.call_fn_wv("nvim_get_api_info".into(), [();0]).await
}
///                                                      *nvim_set_client_info()*
///nvim_set_client_info({name}, {version}, {type}, {methods}, {attributes})
///    Self-identifies the client, and sets optional flags on the channel.
///    Defines the `client` object returned by |nvim_get_chan_info()|.
///
///    Clients should call this just after connecting, to provide hints for
///    debugging and orchestration. (Note: Something is better than nothing!
///    Fields are optional, but at least set `name`.)
///
///    Can be called more than once; the caller should merge old info if
///    appropriate. Example: library first identifies the channel, then a plugin
///    using that library later identifies itself.
///
///    Attributes: ~
///        |RPC| only
///        Since: 0.3.0
///
///    Parameters: ~
///      • {name}        Client short-name. Sets the `client.name` field of
///                      |nvim_get_chan_info()|.
///      • {version}     Dict describing the version, with these (optional) keys:
///                      • "major" major version (defaults to 0 if not set, for
///                        no release yet)
///                      • "minor" minor version
///                      • "patch" patch number
///                      • "prerelease" string describing a prerelease, like
///                        "dev" or "beta1"
///                      • "commit" hash or similar identifier of commit
///      • {type}        Must be one of the following values. Client libraries
///                      should default to "remote" unless overridden by the
///                      user.
///                      • "remote" remote client connected "Nvim flavored"
///                        MessagePack-RPC (responses must be in reverse order of
///                        requests). |msgpack-rpc|
///                      • "msgpack-rpc" remote client connected to Nvim via
///                        fully MessagePack-RPC compliant protocol.
///                      • "ui" gui frontend
///                      • "embedder" application using Nvim as a component (for
///                        example, IDE/editor implementing a vim mode).
///                      • "host" plugin host, typically started by nvim
///                      • "plugin" single plugin, started by nvim
///      • {methods}     Builtin methods in the client. For a host, this does not
///                      include plugin methods which will be discovered later.
///                      The key should be the method name, the values are dicts
///                      with these (optional) keys (more keys may be added in
///                      future versions of Nvim, thus unknown keys are ignored.
///                      Clients must only use keys defined in this or later
///                      versions of Nvim):
///                      • "async" if true, send as a notification. If false or
///                        unspecified, use a blocking request
///                      • "nargs" Number of arguments. Could be a single integer
///                        or an array of two integers, minimum and maximum
///                        inclusive.
///      • {attributes}  Arbitrary string:string map of informal client
///                      properties. Suggested keys:
///                      • "pid": Process id.
///                      • "website": Client homepage URL (e.g. GitHub
///                        repository)
///                      • "license": License description ("Apache 2", "GPLv3",
///                        "MIT", …)
///                      • "logo": URI or path to image, preferably small logo or
///                        icon. .png or .svg format is preferred.
///
async fn set_client_info(&self, name: &str, version: impl Serialize, type_: &str, methods: impl Serialize, attributes: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_set_client_info".into(), (name, version, type_, methods, attributes, )).await
}
///                                                      *nvim_set_client_info()*
///nvim_set_client_info({name}, {version}, {type}, {methods}, {attributes})
///    Self-identifies the client, and sets optional flags on the channel.
///    Defines the `client` object returned by |nvim_get_chan_info()|.
///
///    Clients should call this just after connecting, to provide hints for
///    debugging and orchestration. (Note: Something is better than nothing!
///    Fields are optional, but at least set `name`.)
///
///    Can be called more than once; the caller should merge old info if
///    appropriate. Example: library first identifies the channel, then a plugin
///    using that library later identifies itself.
///
///    Attributes: ~
///        |RPC| only
///        Since: 0.3.0
///
///    Parameters: ~
///      • {name}        Client short-name. Sets the `client.name` field of
///                      |nvim_get_chan_info()|.
///      • {version}     Dict describing the version, with these (optional) keys:
///                      • "major" major version (defaults to 0 if not set, for
///                        no release yet)
///                      • "minor" minor version
///                      • "patch" patch number
///                      • "prerelease" string describing a prerelease, like
///                        "dev" or "beta1"
///                      • "commit" hash or similar identifier of commit
///      • {type}        Must be one of the following values. Client libraries
///                      should default to "remote" unless overridden by the
///                      user.
///                      • "remote" remote client connected "Nvim flavored"
///                        MessagePack-RPC (responses must be in reverse order of
///                        requests). |msgpack-rpc|
///                      • "msgpack-rpc" remote client connected to Nvim via
///                        fully MessagePack-RPC compliant protocol.
///                      • "ui" gui frontend
///                      • "embedder" application using Nvim as a component (for
///                        example, IDE/editor implementing a vim mode).
///                      • "host" plugin host, typically started by nvim
///                      • "plugin" single plugin, started by nvim
///      • {methods}     Builtin methods in the client. For a host, this does not
///                      include plugin methods which will be discovered later.
///                      The key should be the method name, the values are dicts
///                      with these (optional) keys (more keys may be added in
///                      future versions of Nvim, thus unknown keys are ignored.
///                      Clients must only use keys defined in this or later
///                      versions of Nvim):
///                      • "async" if true, send as a notification. If false or
///                        unspecified, use a blocking request
///                      • "nargs" Number of arguments. Could be a single integer
///                        or an array of two integers, minimum and maximum
///                        inclusive.
///      • {attributes}  Arbitrary string:string map of informal client
///                      properties. Suggested keys:
///                      • "pid": Process id.
///                      • "website": Client homepage URL (e.g. GitHub
///                        repository)
///                      • "license": License description ("Apache 2", "GPLv3",
///                        "MIT", …)
///                      • "logo": URI or path to image, preferably small logo or
///                        icon. .png or .svg format is preferred.
///
async fn set_client_info_wv(&self, name: String, version: Dict, type_: String, methods: Dict, attributes: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_set_client_info".into(), (name, version, type_, methods, attributes, )).await
}
///nvim_get_chan_info({chan})                              *nvim_get_chan_info()*
///    Gets information about a channel.
///
///    See |nvim_list_uis()| for an example of how to get channel info.
///
///    Attributes: ~
///        Since: 0.3.0
///
///    Parameters: ~
///      • {chan}  channel_id, or 0 for current channel
///
///    Return: ~
///        Channel info dict with these keys:
///        • "id" Channel id.
///        • "argv" (optional) Job arguments list.
///        • "stream" Stream underlying the channel.
///          • "stdio" stdin and stdout of this Nvim instance
///          • "stderr" stderr of this Nvim instance
///          • "socket" TCP/IP socket or named pipe
///          • "job" Job with communication over its stdio.
///        • "mode" How data received on the channel is interpreted.
///          • "bytes" Send and receive raw bytes.
///          • "terminal" |terminal| instance interprets ASCII sequences.
///          • "rpc" |RPC| communication on the channel is active.
///        • "pty" (optional) Name of pseudoterminal. On a POSIX system this is a
///          device path like "/dev/pts/1". If unknown, the key will still be
///          present if a pty is used (e.g. for conpty on Windows).
///        • "buffer" (optional) Buffer connected to |terminal| instance.
///        • "client" (optional) Info about the peer (client on the other end of
///          the channel), as set by |nvim_set_client_info()|.
///
async fn get_chan_info<D: Deserialize<'static>>(&self, chan: Integer, ) -> error::Result<D> {
	self.call_fn("nvim_get_chan_info".into(), (chan, )).await
}
///nvim_get_chan_info({chan})                              *nvim_get_chan_info()*
///    Gets information about a channel.
///
///    See |nvim_list_uis()| for an example of how to get channel info.
///
///    Attributes: ~
///        Since: 0.3.0
///
///    Parameters: ~
///      • {chan}  channel_id, or 0 for current channel
///
///    Return: ~
///        Channel info dict with these keys:
///        • "id" Channel id.
///        • "argv" (optional) Job arguments list.
///        • "stream" Stream underlying the channel.
///          • "stdio" stdin and stdout of this Nvim instance
///          • "stderr" stderr of this Nvim instance
///          • "socket" TCP/IP socket or named pipe
///          • "job" Job with communication over its stdio.
///        • "mode" How data received on the channel is interpreted.
///          • "bytes" Send and receive raw bytes.
///          • "terminal" |terminal| instance interprets ASCII sequences.
///          • "rpc" |RPC| communication on the channel is active.
///        • "pty" (optional) Name of pseudoterminal. On a POSIX system this is a
///          device path like "/dev/pts/1". If unknown, the key will still be
///          present if a pty is used (e.g. for conpty on Windows).
///        • "buffer" (optional) Buffer connected to |terminal| instance.
///        • "client" (optional) Info about the peer (client on the other end of
///          the channel), as set by |nvim_set_client_info()|.
///
async fn get_chan_info_wv(&self, chan: Integer, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_get_chan_info".into(), (chan, )).await
}
///nvim_list_chans()                                          *nvim_list_chans()*
///    Get information about all open channels.
///
///    Attributes: ~
///        Since: 0.3.0
///
///    Return: ~
///        Array of Dictionaries, each describing a channel with the format
///        specified at |nvim_get_chan_info()|.
///
async fn list_chans<D: Deserialize<'static>>(&self, ) -> error::Result<D> {
	self.call_fn("nvim_list_chans".into(), [();0]).await
}
///nvim_list_chans()                                          *nvim_list_chans()*
///    Get information about all open channels.
///
///    Attributes: ~
///        Since: 0.3.0
///
///    Return: ~
///        Array of Dictionaries, each describing a channel with the format
///        specified at |nvim_get_chan_info()|.
///
async fn list_chans_wv(&self, ) -> error::Result<Array> {
	self.call_fn_wv("nvim_list_chans".into(), [();0]).await
}
///nvim_list_uis()                                              *nvim_list_uis()*
///    Gets a list of dictionaries representing attached UIs.
///
///    Example: The Nvim builtin |TUI| sets its channel info as described in
///    |startup-tui|. In particular, it sets `client.name` to "nvim-tui". So you
///    can check if the TUI is running by inspecting the client name of each UI: >lua
///        vim.print(vim.api.nvim_get_chan_info(vim.api.nvim_list_uis()[1].chan).client.name)
///<
///
///    Attributes: ~
///        Since: 0.3.0
///
///    Return: ~
///        Array of UI dictionaries, each with these keys:
///        • "height" Requested height of the UI
///        • "width" Requested width of the UI
///        • "rgb" true if the UI uses RGB colors (false implies |cterm-colors|)
///        • "ext_..." Requested UI extensions, see |ui-option|
///        • "chan" |channel-id| of remote UI
///
async fn list_uis<D: Deserialize<'static>>(&self, ) -> error::Result<D> {
	self.call_fn("nvim_list_uis".into(), [();0]).await
}
///nvim_list_uis()                                              *nvim_list_uis()*
///    Gets a list of dictionaries representing attached UIs.
///
///    Example: The Nvim builtin |TUI| sets its channel info as described in
///    |startup-tui|. In particular, it sets `client.name` to "nvim-tui". So you
///    can check if the TUI is running by inspecting the client name of each UI: >lua
///        vim.print(vim.api.nvim_get_chan_info(vim.api.nvim_list_uis()[1].chan).client.name)
///<
///
///    Attributes: ~
///        Since: 0.3.0
///
///    Return: ~
///        Array of UI dictionaries, each with these keys:
///        • "height" Requested height of the UI
///        • "width" Requested width of the UI
///        • "rgb" true if the UI uses RGB colors (false implies |cterm-colors|)
///        • "ext_..." Requested UI extensions, see |ui-option|
///        • "chan" |channel-id| of remote UI
///
async fn list_uis_wv(&self, ) -> error::Result<Array> {
	self.call_fn_wv("nvim_list_uis".into(), [();0]).await
}
///nvim_get_proc_children({pid})                       *nvim_get_proc_children()*
///    Gets the immediate children of process `pid`.
///
///    Attributes: ~
///        Since: 0.3.0
///
///    Return: ~
///        Array of child process ids, empty if process not found.
///
async fn get_proc_children<D: Deserialize<'static>>(&self, pid: Integer, ) -> error::Result<D> {
	self.call_fn("nvim_get_proc_children".into(), (pid, )).await
}
///nvim_get_proc_children({pid})                       *nvim_get_proc_children()*
///    Gets the immediate children of process `pid`.
///
///    Attributes: ~
///        Since: 0.3.0
///
///    Return: ~
///        Array of child process ids, empty if process not found.
///
async fn get_proc_children_wv(&self, pid: Integer, ) -> error::Result<Array> {
	self.call_fn_wv("nvim_get_proc_children".into(), (pid, )).await
}
///nvim_get_proc({pid})                                         *nvim_get_proc()*
///    Gets info describing process `pid`.
///
///    Attributes: ~
///        Since: 0.3.0
///
///    Return: ~
///        Map of process properties, or NIL if process not found.
///
async fn get_proc<D: Deserialize<'static>>(&self, pid: Integer, ) -> error::Result<D> {
	self.call_fn("nvim_get_proc".into(), (pid, )).await
}
///nvim_get_proc({pid})                                         *nvim_get_proc()*
///    Gets info describing process `pid`.
///
///    Attributes: ~
///        Since: 0.3.0
///
///    Return: ~
///        Map of process properties, or NIL if process not found.
///
async fn get_proc_wv(&self, pid: Integer, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_get_proc".into(), (pid, )).await
}
///                                                *nvim_select_popupmenu_item()*
///nvim_select_popupmenu_item({item}, {insert}, {finish}, {opts})
///    Selects an item in the completion popup menu.
///
///    If neither |ins-completion| nor |cmdline-completion| popup menu is active
///    this API call is silently ignored. Useful for an external UI using
///    |ui-popupmenu| to control the popup menu with the mouse. Can also be used
///    in a mapping; use <Cmd> |:map-cmd| or a Lua mapping to ensure the mapping
///    doesn't end completion mode.
///
///    Attributes: ~
///        Since: 0.4.0
///
///    Parameters: ~
///      • {item}    Index (zero-based) of the item to select. Value of -1
///                  selects nothing and restores the original text.
///      • {insert}  For |ins-completion|, whether the selection should be
///                  inserted in the buffer. Ignored for |cmdline-completion|.
///      • {finish}  Finish the completion and dismiss the popup menu. Implies
///                  {insert}.
///      • {opts}    Optional parameters. Reserved for future use.
///
async fn select_popupmenu_item(&self, item: Integer, insert: Boolean, finish: Boolean, opts: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_select_popupmenu_item".into(), (item, insert, finish, opts, )).await
}
///                                                *nvim_select_popupmenu_item()*
///nvim_select_popupmenu_item({item}, {insert}, {finish}, {opts})
///    Selects an item in the completion popup menu.
///
///    If neither |ins-completion| nor |cmdline-completion| popup menu is active
///    this API call is silently ignored. Useful for an external UI using
///    |ui-popupmenu| to control the popup menu with the mouse. Can also be used
///    in a mapping; use <Cmd> |:map-cmd| or a Lua mapping to ensure the mapping
///    doesn't end completion mode.
///
///    Attributes: ~
///        Since: 0.4.0
///
///    Parameters: ~
///      • {item}    Index (zero-based) of the item to select. Value of -1
///                  selects nothing and restores the original text.
///      • {insert}  For |ins-completion|, whether the selection should be
///                  inserted in the buffer. Ignored for |cmdline-completion|.
///      • {finish}  Finish the completion and dismiss the popup menu. Implies
///                  {insert}.
///      • {opts}    Optional parameters. Reserved for future use.
///
async fn select_popupmenu_item_wv(&self, item: Integer, insert: Boolean, finish: Boolean, opts: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_select_popupmenu_item".into(), (item, insert, finish, opts, )).await
}
///nvim_del_mark({name})                                        *nvim_del_mark()*
///    Deletes an uppercase/file named mark. See |mark-motions|.
///
///    Note: ~
///      • Lowercase name (or other buffer-local mark) is an error.
///
///    Attributes: ~
///        Since: 0.6.0
///
///    Parameters: ~
///      • {name}  Mark name
///
///    Return: ~
///        true if the mark was deleted, else false.
///
///    See also: ~
///      • |nvim_buf_del_mark()|
///      • |nvim_get_mark()|
///
async fn del_mark(&self, name: &str, ) -> error::Result<Boolean> {
	self.call_fn("nvim_del_mark".into(), (name, )).await
}
///nvim_get_mark({name}, {opts})                                *nvim_get_mark()*
///    Returns a `(row, col, buffer, buffername)` tuple representing the position
///    of the uppercase/file named mark. "End of line" column position is
///    returned as |v:maxcol| (big number). See |mark-motions|.
///
///    Marks are (1,0)-indexed. |api-indexing|
///
///    Note: ~
///      • Lowercase name (or other buffer-local mark) is an error.
///
///    Attributes: ~
///        Since: 0.6.0
///
///    Parameters: ~
///      • {name}  Mark name
///      • {opts}  Optional parameters. Reserved for future use.
///
///    Return: ~
///        4-tuple (row, col, buffer, buffername), (0, 0, 0, '') if the mark is
///        not set.
///
///    See also: ~
///      • |nvim_buf_set_mark()|
///      • |nvim_del_mark()|
///
async fn get_mark<D: Deserialize<'static>>(&self, name: &str, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_get_mark".into(), (name, opts, )).await
}
///nvim_get_mark({name}, {opts})                                *nvim_get_mark()*
///    Returns a `(row, col, buffer, buffername)` tuple representing the position
///    of the uppercase/file named mark. "End of line" column position is
///    returned as |v:maxcol| (big number). See |mark-motions|.
///
///    Marks are (1,0)-indexed. |api-indexing|
///
///    Note: ~
///      • Lowercase name (or other buffer-local mark) is an error.
///
///    Attributes: ~
///        Since: 0.6.0
///
///    Parameters: ~
///      • {name}  Mark name
///      • {opts}  Optional parameters. Reserved for future use.
///
///    Return: ~
///        4-tuple (row, col, buffer, buffername), (0, 0, 0, '') if the mark is
///        not set.
///
///    See also: ~
///      • |nvim_buf_set_mark()|
///      • |nvim_del_mark()|
///
async fn get_mark_wv(&self, name: String, opts: Dict, ) -> error::Result<Array> {
	self.call_fn_wv("nvim_get_mark".into(), (name, opts, )).await
}
///nvim_eval_statusline({str}, {opts})                   *nvim_eval_statusline()*
///    Evaluates statusline string.
///
///    Attributes: ~
///        |api-fast|
///        Since: 0.6.0
///
///    Parameters: ~
///      • {str}   Statusline string (see 'statusline').
///      • {opts}  Optional parameters.
///                • winid: (number) |window-ID| of the window to use as context
///                  for statusline.
///                • maxwidth: (number) Maximum width of statusline.
///                • fillchar: (string) Character to fill blank spaces in the
///                  statusline (see 'fillchars'). Treated as single-width even
///                  if it isn't.
///                • highlights: (boolean) Return highlight information.
///                • use_winbar: (boolean) Evaluate winbar instead of statusline.
///                • use_tabline: (boolean) Evaluate tabline instead of
///                  statusline. When true, {winid} is ignored. Mutually
///                  exclusive with {use_winbar}.
///                • use_statuscol_lnum: (number) Evaluate statuscolumn for this
///                  line number instead of statusline.
///
///    Return: ~
///        Dict containing statusline information, with these keys:
///        • str: (string) Characters that will be displayed on the statusline.
///        • width: (number) Display width of the statusline.
///        • highlights: Array containing highlight information of the
///          statusline. Only included when the "highlights" key in {opts} is
///          true. Each element of the array is a |Dict| with these keys:
///          • start: (number) Byte index (0-based) of first character that uses
///            the highlight.
///          • group: (string) Deprecated. Use `groups` instead.
///          • groups: (array) Names of stacked highlight groups (highest
///            priority last).
///
async fn eval_statusline<D: Deserialize<'static>>(&self, str: &str, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_eval_statusline".into(), (str, opts, )).await
}
///nvim_eval_statusline({str}, {opts})                   *nvim_eval_statusline()*
///    Evaluates statusline string.
///
///    Attributes: ~
///        |api-fast|
///        Since: 0.6.0
///
///    Parameters: ~
///      • {str}   Statusline string (see 'statusline').
///      • {opts}  Optional parameters.
///                • winid: (number) |window-ID| of the window to use as context
///                  for statusline.
///                • maxwidth: (number) Maximum width of statusline.
///                • fillchar: (string) Character to fill blank spaces in the
///                  statusline (see 'fillchars'). Treated as single-width even
///                  if it isn't.
///                • highlights: (boolean) Return highlight information.
///                • use_winbar: (boolean) Evaluate winbar instead of statusline.
///                • use_tabline: (boolean) Evaluate tabline instead of
///                  statusline. When true, {winid} is ignored. Mutually
///                  exclusive with {use_winbar}.
///                • use_statuscol_lnum: (number) Evaluate statuscolumn for this
///                  line number instead of statusline.
///
///    Return: ~
///        Dict containing statusline information, with these keys:
///        • str: (string) Characters that will be displayed on the statusline.
///        • width: (number) Display width of the statusline.
///        • highlights: Array containing highlight information of the
///          statusline. Only included when the "highlights" key in {opts} is
///          true. Each element of the array is a |Dict| with these keys:
///          • start: (number) Byte index (0-based) of first character that uses
///            the highlight.
///          • group: (string) Deprecated. Use `groups` instead.
///          • groups: (array) Names of stacked highlight groups (highest
///            priority last).
///
async fn eval_statusline_wv(&self, str: String, opts: Dict, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_eval_statusline".into(), (str, opts, )).await
}
///nvim_exec2({src}, {opts})                                       *nvim_exec2()*
///    Executes Vimscript (multiline block of Ex commands), like anonymous
///    |:source|.
///
///    Unlike |nvim_command()| this function supports heredocs, script-scope
///    (s:), etc.
///
///    On execution error: fails with Vimscript error, updates v:errmsg.
///
///    Attributes: ~
///        Since: 0.9.0
///
///    Parameters: ~
///      • {src}   Vimscript code
///      • {opts}  Optional parameters.
///                • output: (boolean, default false) Whether to capture and
///                  return all (non-error, non-shell |:!|) output.
///
///    Return: ~
///        Dict containing information about execution, with these keys:
///        • output: (string|nil) Output if `opts.output` is true.
///
///    See also: ~
///      • |execute()|
///      • |nvim_command()|
///      • |nvim_cmd()|
///
async fn exec2<D: Deserialize<'static>>(&self, src: &str, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_exec2".into(), (src, opts, )).await
}
///nvim_exec2({src}, {opts})                                       *nvim_exec2()*
///    Executes Vimscript (multiline block of Ex commands), like anonymous
///    |:source|.
///
///    Unlike |nvim_command()| this function supports heredocs, script-scope
///    (s:), etc.
///
///    On execution error: fails with Vimscript error, updates v:errmsg.
///
///    Attributes: ~
///        Since: 0.9.0
///
///    Parameters: ~
///      • {src}   Vimscript code
///      • {opts}  Optional parameters.
///                • output: (boolean, default false) Whether to capture and
///                  return all (non-error, non-shell |:!|) output.
///
///    Return: ~
///        Dict containing information about execution, with these keys:
///        • output: (string|nil) Output if `opts.output` is true.
///
///    See also: ~
///      • |execute()|
///      • |nvim_command()|
///      • |nvim_cmd()|
///
async fn exec2_wv(&self, src: String, opts: Dict, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_exec2".into(), (src, opts, )).await
}
///nvim_command({command})                                       *nvim_command()*
///    Executes an Ex command.
///
///    On execution error: fails with Vimscript error, updates v:errmsg.
///
///    Prefer |nvim_cmd()| or |nvim_exec2()| instead. To modify an Ex command in
///    a structured way before executing it, modify the result of
///    |nvim_parse_cmd()| then pass it to |nvim_cmd()|.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {command}  Ex command string
///
async fn command(&self, command: &str, ) -> error::Result<()> {
	self.call_fn("nvim_command".into(), (command, )).await
}
///nvim_eval({expr})                                                *nvim_eval()*
///    Evaluates a Vimscript |expression|. Dicts and Lists are recursively
///    expanded.
///
///    On execution error: fails with Vimscript error, updates v:errmsg.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {expr}  Vimscript expression string
///
///    Return: ~
///        Evaluation result or expanded object
///
async fn eval<D: Deserialize<'static>>(&self, expr: &str, ) -> error::Result<D> {
	self.call_fn("nvim_eval".into(), (expr, )).await
}
///nvim_eval({expr})                                                *nvim_eval()*
///    Evaluates a Vimscript |expression|. Dicts and Lists are recursively
///    expanded.
///
///    On execution error: fails with Vimscript error, updates v:errmsg.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {expr}  Vimscript expression string
///
///    Return: ~
///        Evaluation result or expanded object
///
async fn eval_wv(&self, expr: String, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_eval".into(), (expr, )).await
}
///nvim_call_function({fn}, {args})                        *nvim_call_function()*
///    Calls a Vimscript function with the given arguments.
///
///    On execution error: fails with Vimscript error, updates v:errmsg.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {fn}    Function to call
///      • {args}  Function arguments packed in an Array
///
///    Return: ~
///        Result of the function call
///
async fn call_function<D: Deserialize<'static>>(&self, fn_: &str, args: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_call_function".into(), (fn_, args, )).await
}
///nvim_call_function({fn}, {args})                        *nvim_call_function()*
///    Calls a Vimscript function with the given arguments.
///
///    On execution error: fails with Vimscript error, updates v:errmsg.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {fn}    Function to call
///      • {args}  Function arguments packed in an Array
///
///    Return: ~
///        Result of the function call
///
async fn call_function_wv(&self, fn_: String, args: Array, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_call_function".into(), (fn_, args, )).await
}
///                                                   *nvim_call_dict_function()*
///nvim_call_dict_function({dict}, {fn}, {args})
///    Calls a Vimscript |Dictionary-function| with the given arguments.
///
///    On execution error: fails with Vimscript error, updates v:errmsg.
///
///    Attributes: ~
///        Since: 0.3.0
///
///    Parameters: ~
///      • {dict}  Dict, or String evaluating to a Vimscript |self| dict
///      • {fn}    Name of the function defined on the Vimscript dict
///      • {args}  Function arguments packed in an Array
///
///    Return: ~
///        Result of the function call
///
async fn call_dict_function<D: Deserialize<'static>>(&self, dict: impl Serialize, fn_: &str, args: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_call_dict_function".into(), (dict, fn_, args, )).await
}
///                                                   *nvim_call_dict_function()*
///nvim_call_dict_function({dict}, {fn}, {args})
///    Calls a Vimscript |Dictionary-function| with the given arguments.
///
///    On execution error: fails with Vimscript error, updates v:errmsg.
///
///    Attributes: ~
///        Since: 0.3.0
///
///    Parameters: ~
///      • {dict}  Dict, or String evaluating to a Vimscript |self| dict
///      • {fn}    Name of the function defined on the Vimscript dict
///      • {args}  Function arguments packed in an Array
///
///    Return: ~
///        Result of the function call
///
async fn call_dict_function_wv(&self, dict: Object, fn_: String, args: Array, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_call_dict_function".into(), (dict, fn_, args, )).await
}
///                                                     *nvim_parse_expression()*
///nvim_parse_expression({expr}, {flags}, {highlight})
///    Parse a Vimscript expression.
///
///    Attributes: ~
///        |api-fast|
///        Since: 0.3.0
///
///    Parameters: ~
///      • {expr}       Expression to parse. Always treated as a single line.
///      • {flags}      Flags:
///                     • "m" if multiple expressions in a row are allowed (only
///                       the first one will be parsed),
///                     • "E" if EOC tokens are not allowed (determines whether
///                       they will stop parsing process or be recognized as an
///                       operator/space, though also yielding an error).
///                     • "l" when needing to start parsing with lvalues for
///                       ":let" or ":for". Common flag sets:
///                     • "m" to parse like for `":echo"`.
///                     • "E" to parse like for `"<C-r>="`.
///                     • empty string for ":call".
///                     • "lm" to parse for ":let".
///      • {highlight}  If true, return value will also include "highlight" key
///                     containing array of 4-tuples (arrays) (Integer, Integer,
///                     Integer, String), where first three numbers define the
///                     highlighted region and represent line, starting column
///                     and ending column (latter exclusive: one should highlight
///                     region [start_col, end_col)).
///
///    Return: ~
///        • AST: top-level dict with these keys:
///          • "error": Dict with error, present only if parser saw some error.
///            Contains the following keys:
///            • "message": String, error message in printf format, translated.
///              Must contain exactly one "%.*s".
///            • "arg": String, error message argument.
///          • "len": Amount of bytes successfully parsed. With flags equal to ""
///            that should be equal to the length of expr string. ("Successfully
///            parsed" here means "participated in AST creation", not "till the
///            first error".)
///          • "ast": AST, either nil or a dict with these keys:
///            • "type": node type, one of the value names from ExprASTNodeType
///              stringified without "kExprNode" prefix.
///            • "start": a pair `[line, column]` describing where node is
///              "started" where "line" is always 0 (will not be 0 if you will be
///              using this API on e.g. ":let", but that is not present yet).
///              Both elements are Integers.
///            • "len": “length” of the node. This and "start" are there for
///              debugging purposes primary (debugging parser and providing debug
///              information).
///            • "children": a list of nodes described in top/"ast". There always
///              is zero, one or two children, key will not be present if node
///              has no children. Maximum number of children may be found in
///              node_maxchildren array.
///        • Local values (present only for certain nodes):
///          • "scope": a single Integer, specifies scope for "Option" and
///            "PlainIdentifier" nodes. For "Option" it is one of ExprOptScope
///            values, for "PlainIdentifier" it is one of ExprVarScope values.
///          • "ident": identifier (without scope, if any), present for "Option",
///            "PlainIdentifier", "PlainKey" and "Environment" nodes.
///          • "name": Integer, register name (one character) or -1. Only present
///            for "Register" nodes.
///          • "cmp_type": String, comparison type, one of the value names from
///            ExprComparisonType, stringified without "kExprCmp" prefix. Only
///            present for "Comparison" nodes.
///          • "ccs_strategy": String, case comparison strategy, one of the value
///            names from ExprCaseCompareStrategy, stringified without
///            "kCCStrategy" prefix. Only present for "Comparison" nodes.
///          • "augmentation": String, augmentation type for "Assignment" nodes.
///            Is either an empty string, "Add", "Subtract" or "Concat" for "=",
///            "+=", "-=" or ".=" respectively.
///          • "invert": Boolean, true if result of comparison needs to be
///            inverted. Only present for "Comparison" nodes.
///          • "ivalue": Integer, integer value for "Integer" nodes.
///          • "fvalue": Float, floating-point value for "Float" nodes.
///          • "svalue": String, value for "SingleQuotedString" and
///            "DoubleQuotedString" nodes.
///
///
///==============================================================================
async fn parse_expression<D: Deserialize<'static>>(&self, expr: &str, flags: &str, highlight: Boolean, ) -> error::Result<D> {
	self.call_fn("nvim_parse_expression".into(), (expr, flags, highlight, )).await
}
///                                                     *nvim_parse_expression()*
///nvim_parse_expression({expr}, {flags}, {highlight})
///    Parse a Vimscript expression.
///
///    Attributes: ~
///        |api-fast|
///        Since: 0.3.0
///
///    Parameters: ~
///      • {expr}       Expression to parse. Always treated as a single line.
///      • {flags}      Flags:
///                     • "m" if multiple expressions in a row are allowed (only
///                       the first one will be parsed),
///                     • "E" if EOC tokens are not allowed (determines whether
///                       they will stop parsing process or be recognized as an
///                       operator/space, though also yielding an error).
///                     • "l" when needing to start parsing with lvalues for
///                       ":let" or ":for". Common flag sets:
///                     • "m" to parse like for `":echo"`.
///                     • "E" to parse like for `"<C-r>="`.
///                     • empty string for ":call".
///                     • "lm" to parse for ":let".
///      • {highlight}  If true, return value will also include "highlight" key
///                     containing array of 4-tuples (arrays) (Integer, Integer,
///                     Integer, String), where first three numbers define the
///                     highlighted region and represent line, starting column
///                     and ending column (latter exclusive: one should highlight
///                     region [start_col, end_col)).
///
///    Return: ~
///        • AST: top-level dict with these keys:
///          • "error": Dict with error, present only if parser saw some error.
///            Contains the following keys:
///            • "message": String, error message in printf format, translated.
///              Must contain exactly one "%.*s".
///            • "arg": String, error message argument.
///          • "len": Amount of bytes successfully parsed. With flags equal to ""
///            that should be equal to the length of expr string. ("Successfully
///            parsed" here means "participated in AST creation", not "till the
///            first error".)
///          • "ast": AST, either nil or a dict with these keys:
///            • "type": node type, one of the value names from ExprASTNodeType
///              stringified without "kExprNode" prefix.
///            • "start": a pair `[line, column]` describing where node is
///              "started" where "line" is always 0 (will not be 0 if you will be
///              using this API on e.g. ":let", but that is not present yet).
///              Both elements are Integers.
///            • "len": “length” of the node. This and "start" are there for
///              debugging purposes primary (debugging parser and providing debug
///              information).
///            • "children": a list of nodes described in top/"ast". There always
///              is zero, one or two children, key will not be present if node
///              has no children. Maximum number of children may be found in
///              node_maxchildren array.
///        • Local values (present only for certain nodes):
///          • "scope": a single Integer, specifies scope for "Option" and
///            "PlainIdentifier" nodes. For "Option" it is one of ExprOptScope
///            values, for "PlainIdentifier" it is one of ExprVarScope values.
///          • "ident": identifier (without scope, if any), present for "Option",
///            "PlainIdentifier", "PlainKey" and "Environment" nodes.
///          • "name": Integer, register name (one character) or -1. Only present
///            for "Register" nodes.
///          • "cmp_type": String, comparison type, one of the value names from
///            ExprComparisonType, stringified without "kExprCmp" prefix. Only
///            present for "Comparison" nodes.
///          • "ccs_strategy": String, case comparison strategy, one of the value
///            names from ExprCaseCompareStrategy, stringified without
///            "kCCStrategy" prefix. Only present for "Comparison" nodes.
///          • "augmentation": String, augmentation type for "Assignment" nodes.
///            Is either an empty string, "Add", "Subtract" or "Concat" for "=",
///            "+=", "-=" or ".=" respectively.
///          • "invert": Boolean, true if result of comparison needs to be
///            inverted. Only present for "Comparison" nodes.
///          • "ivalue": Integer, integer value for "Integer" nodes.
///          • "fvalue": Float, floating-point value for "Float" nodes.
///          • "svalue": String, value for "SingleQuotedString" and
///            "DoubleQuotedString" nodes.
///
///
///==============================================================================
async fn parse_expression_wv(&self, expr: String, flags: String, highlight: Boolean, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_parse_expression".into(), (expr, flags, highlight, )).await
}
///nvim_open_win({buffer}, {enter}, {config})                   *nvim_open_win()*
///    Opens a new split window, or a floating window if `relative` is specified,
///    or an external window (managed by the UI) if `external` is specified.
///
///    Floats are windows that are drawn above the split layout, at some anchor
///    position in some other window. Floats can be drawn internally or by
///    external GUI with the |ui-multigrid| extension. External windows are only
///    supported with multigrid GUIs, and are displayed as separate top-level
///    windows.
///
///    For a general overview of floats, see |api-floatwin|.
///
///    The `width` and `height` of the new window must be specified when opening
///    a floating window, but are optional for normal windows.
///
///    If `relative` and `external` are omitted, a normal "split" window is
///    created. The `win` property determines which window will be split. If no
///    `win` is provided or `win == 0`, a window will be created adjacent to the
///    current window. If -1 is provided, a top-level split will be created.
///    `vertical` and `split` are only valid for normal windows, and are used to
///    control split direction. For `vertical`, the exact direction is determined
///    by |'splitright'| and |'splitbelow'|. Split windows cannot have
///    `bufpos`/`row`/`col`/`border`/`title`/`footer` properties.
///
///    With relative=editor (row=0,col=0) refers to the top-left corner of the
///    screen-grid and (row=Lines-1,col=Columns-1) refers to the bottom-right
///    corner. Fractional values are allowed, but the builtin implementation
///    (used by non-multigrid UIs) will always round down to nearest integer.
///
///    Out-of-bounds values, and configurations that make the float not fit
///    inside the main editor, are allowed. The builtin implementation truncates
///    values so floats are fully within the main screen grid. External GUIs
///    could let floats hover outside of the main window like a tooltip, but this
///    should not be used to specify arbitrary WM screen positions.
///
///    Example (Lua): window-relative float >lua
///        vim.api.nvim_open_win(0, false,
///          {relative='win', row=3, col=3, width=12, height=3})
///<
///
///    Example (Lua): buffer-relative float (travels as buffer is scrolled) >lua
///        vim.api.nvim_open_win(0, false,
///          {relative='win', width=12, height=3, bufpos={100,10}})
///<
///
///    Example (Lua): vertical split left of the current window >lua
///        vim.api.nvim_open_win(0, false, {
///          split = 'left',
///          win = 0
///        })
///<
///
///    Attributes: ~
///        not allowed when |textlock| is active
///        Since: 0.4.0
///
///    Parameters: ~
///      • {buffer}  Buffer to display, or 0 for current buffer
///      • {enter}   Enter the window (make it the current window)
///      • {config}  Map defining the window configuration. Keys:
///                  • relative: Sets the window layout to "floating", placed at
///                    (row,col) coordinates relative to:
///                    • "cursor" Cursor position in current window.
///                    • "editor" The global editor grid.
///                    • "laststatus" 'laststatus' if present, or last row.
///                    • "mouse" Mouse position.
///                    • "tabline" Tabline if present, or first row.
///                    • "win" Window given by the `win` field, or current
///                      window.
///                  • win: |window-ID| window to split, or relative window when
///                    creating a float (relative="win").
///                  • anchor: Decides which corner of the float to place at
///                    (row,col):
///                    • "NW" northwest (default)
///                    • "NE" northeast
///                    • "SW" southwest
///                    • "SE" southeast
///                  • width: Window width (in character cells). Minimum of 1.
///                  • height: Window height (in character cells). Minimum of 1.
///                  • bufpos: Places float relative to buffer text (only when
///                    relative="win"). Takes a tuple of zero-indexed
///                    `[line, column]`. `row` and `col` if given are applied
///                    relative to this position, else they default to:
///                    • `row=1` and `col=0` if `anchor` is "NW" or "NE"
///                    • `row=0` and `col=0` if `anchor` is "SW" or "SE" (thus
///                      like a tooltip near the buffer text).
///                  • row: Row position in units of "screen cell height", may be
///                    fractional.
///                  • col: Column position in units of "screen cell width", may
///                    be fractional.
///                  • focusable: Enable focus by user actions (wincmds, mouse
///                    events). Defaults to true. Non-focusable windows can be
///                    entered by |nvim_set_current_win()|, or, when the `mouse`
///                    field is set to true, by mouse events. See |focusable|.
///                  • mouse: Specify how this window interacts with mouse
///                    events. Defaults to `focusable` value.
///                    • If false, mouse events pass through this window.
///                    • If true, mouse events interact with this window
///                      normally.
///                  • external: GUI should display the window as an external
///                    top-level window. Currently accepts no other positioning
///                    configuration together with this.
///                  • zindex: Stacking order. floats with higher `zindex` go on
///                    top on floats with lower indices. Must be larger than
///                    zero. The following screen elements have hard-coded
///                    z-indices:
///                    • 100: insert completion popupmenu
///                    • 200: message scrollback
///                    • 250: cmdline completion popupmenu (when
///                      wildoptions+=pum) The default value for floats are 50.
///                      In general, values below 100 are recommended, unless
///                      there is a good reason to overshadow builtin elements.
///                  • style: (optional) Configure the appearance of the window.
///                    Currently only supports one value:
///                    • "minimal" Nvim will display the window with many UI
///                      options disabled. This is useful when displaying a
///                      temporary float where the text should not be edited.
///                      Disables 'number', 'relativenumber', 'cursorline',
///                      'cursorcolumn', 'foldcolumn', 'spell' and 'list'
///                      options. 'signcolumn' is changed to `auto` and
///                      'colorcolumn' is cleared. 'statuscolumn' is changed to
///                      empty. The end-of-buffer region is hidden by setting
///                      `eob` flag of 'fillchars' to a space char, and clearing
///                      the |hl-EndOfBuffer| region in 'winhighlight'.
///                  • border: Style of (optional) window border. This can either
///                    be a string or an array. The string values are the same as
///                    those described in 'winborder'. If it is an array, it
///                    should have a length of eight or any divisor of eight. The
///                    array will specify the eight chars building up the border
///                    in a clockwise fashion starting with the top-left corner.
///                    As an example, the double box style could be specified as: >
///                     [ "╔", "═" ,"╗", "║", "╝", "═", "╚", "║" ].
///<
///                    If the number of chars are less than eight, they will be
///                    repeated. Thus an ASCII border could be specified as >
///                     [ "/", "-", \"\\\\\", "|" ],
///<
///                    or all chars the same as >
///                     [ "x" ].
///<
///                    An empty string can be used to turn off a specific border,
///                    for instance, >
///                     [ "", "", "", ">", "", "", "", "<" ]
///<
///                    will only make vertical borders but not horizontal ones.
///                    By default, `FloatBorder` highlight is used, which links
///                    to `WinSeparator` when not defined. It could also be
///                    specified by character: >
///                     [ ["+", "MyCorner"], ["x", "MyBorder"] ].
///<
///                  • title: Title (optional) in window border, string or list.
///                    List should consist of `[text, highlight]` tuples. If
///                    string, or a tuple lacks a highlight, the default
///                    highlight group is `FloatTitle`.
///                  • title_pos: Title position. Must be set with `title`
///                    option. Value can be one of "left", "center", or "right".
///                    Default is `"left"`.
///                  • footer: Footer (optional) in window border, string or
///                    list. List should consist of `[text, highlight]` tuples.
///                    If string, or a tuple lacks a highlight, the default
///                    highlight group is `FloatFooter`.
///                  • footer_pos: Footer position. Must be set with `footer`
///                    option. Value can be one of "left", "center", or "right".
///                    Default is `"left"`.
///                  • noautocmd: If true then all autocommands are blocked for
///                    the duration of the call.
///                  • fixed: If true when anchor is NW or SW, the float window
///                    would be kept fixed even if the window would be truncated.
///                  • hide: If true the floating window will be hidden.
///                  • vertical: Split vertically |:vertical|.
///                  • split: Split direction: "left", "right", "above", "below".
///
///    Return: ~
///        |window-ID|, or 0 on error
///
async fn open_win(&self, buffer: Buffer, enter: Boolean, config: impl Serialize, ) -> error::Result<Window> {
	self.call_fn("nvim_open_win".into(), (buffer, enter, config, )).await
}
///nvim_open_win({buffer}, {enter}, {config})                   *nvim_open_win()*
///    Opens a new split window, or a floating window if `relative` is specified,
///    or an external window (managed by the UI) if `external` is specified.
///
///    Floats are windows that are drawn above the split layout, at some anchor
///    position in some other window. Floats can be drawn internally or by
///    external GUI with the |ui-multigrid| extension. External windows are only
///    supported with multigrid GUIs, and are displayed as separate top-level
///    windows.
///
///    For a general overview of floats, see |api-floatwin|.
///
///    The `width` and `height` of the new window must be specified when opening
///    a floating window, but are optional for normal windows.
///
///    If `relative` and `external` are omitted, a normal "split" window is
///    created. The `win` property determines which window will be split. If no
///    `win` is provided or `win == 0`, a window will be created adjacent to the
///    current window. If -1 is provided, a top-level split will be created.
///    `vertical` and `split` are only valid for normal windows, and are used to
///    control split direction. For `vertical`, the exact direction is determined
///    by |'splitright'| and |'splitbelow'|. Split windows cannot have
///    `bufpos`/`row`/`col`/`border`/`title`/`footer` properties.
///
///    With relative=editor (row=0,col=0) refers to the top-left corner of the
///    screen-grid and (row=Lines-1,col=Columns-1) refers to the bottom-right
///    corner. Fractional values are allowed, but the builtin implementation
///    (used by non-multigrid UIs) will always round down to nearest integer.
///
///    Out-of-bounds values, and configurations that make the float not fit
///    inside the main editor, are allowed. The builtin implementation truncates
///    values so floats are fully within the main screen grid. External GUIs
///    could let floats hover outside of the main window like a tooltip, but this
///    should not be used to specify arbitrary WM screen positions.
///
///    Example (Lua): window-relative float >lua
///        vim.api.nvim_open_win(0, false,
///          {relative='win', row=3, col=3, width=12, height=3})
///<
///
///    Example (Lua): buffer-relative float (travels as buffer is scrolled) >lua
///        vim.api.nvim_open_win(0, false,
///          {relative='win', width=12, height=3, bufpos={100,10}})
///<
///
///    Example (Lua): vertical split left of the current window >lua
///        vim.api.nvim_open_win(0, false, {
///          split = 'left',
///          win = 0
///        })
///<
///
///    Attributes: ~
///        not allowed when |textlock| is active
///        Since: 0.4.0
///
///    Parameters: ~
///      • {buffer}  Buffer to display, or 0 for current buffer
///      • {enter}   Enter the window (make it the current window)
///      • {config}  Map defining the window configuration. Keys:
///                  • relative: Sets the window layout to "floating", placed at
///                    (row,col) coordinates relative to:
///                    • "cursor" Cursor position in current window.
///                    • "editor" The global editor grid.
///                    • "laststatus" 'laststatus' if present, or last row.
///                    • "mouse" Mouse position.
///                    • "tabline" Tabline if present, or first row.
///                    • "win" Window given by the `win` field, or current
///                      window.
///                  • win: |window-ID| window to split, or relative window when
///                    creating a float (relative="win").
///                  • anchor: Decides which corner of the float to place at
///                    (row,col):
///                    • "NW" northwest (default)
///                    • "NE" northeast
///                    • "SW" southwest
///                    • "SE" southeast
///                  • width: Window width (in character cells). Minimum of 1.
///                  • height: Window height (in character cells). Minimum of 1.
///                  • bufpos: Places float relative to buffer text (only when
///                    relative="win"). Takes a tuple of zero-indexed
///                    `[line, column]`. `row` and `col` if given are applied
///                    relative to this position, else they default to:
///                    • `row=1` and `col=0` if `anchor` is "NW" or "NE"
///                    • `row=0` and `col=0` if `anchor` is "SW" or "SE" (thus
///                      like a tooltip near the buffer text).
///                  • row: Row position in units of "screen cell height", may be
///                    fractional.
///                  • col: Column position in units of "screen cell width", may
///                    be fractional.
///                  • focusable: Enable focus by user actions (wincmds, mouse
///                    events). Defaults to true. Non-focusable windows can be
///                    entered by |nvim_set_current_win()|, or, when the `mouse`
///                    field is set to true, by mouse events. See |focusable|.
///                  • mouse: Specify how this window interacts with mouse
///                    events. Defaults to `focusable` value.
///                    • If false, mouse events pass through this window.
///                    • If true, mouse events interact with this window
///                      normally.
///                  • external: GUI should display the window as an external
///                    top-level window. Currently accepts no other positioning
///                    configuration together with this.
///                  • zindex: Stacking order. floats with higher `zindex` go on
///                    top on floats with lower indices. Must be larger than
///                    zero. The following screen elements have hard-coded
///                    z-indices:
///                    • 100: insert completion popupmenu
///                    • 200: message scrollback
///                    • 250: cmdline completion popupmenu (when
///                      wildoptions+=pum) The default value for floats are 50.
///                      In general, values below 100 are recommended, unless
///                      there is a good reason to overshadow builtin elements.
///                  • style: (optional) Configure the appearance of the window.
///                    Currently only supports one value:
///                    • "minimal" Nvim will display the window with many UI
///                      options disabled. This is useful when displaying a
///                      temporary float where the text should not be edited.
///                      Disables 'number', 'relativenumber', 'cursorline',
///                      'cursorcolumn', 'foldcolumn', 'spell' and 'list'
///                      options. 'signcolumn' is changed to `auto` and
///                      'colorcolumn' is cleared. 'statuscolumn' is changed to
///                      empty. The end-of-buffer region is hidden by setting
///                      `eob` flag of 'fillchars' to a space char, and clearing
///                      the |hl-EndOfBuffer| region in 'winhighlight'.
///                  • border: Style of (optional) window border. This can either
///                    be a string or an array. The string values are the same as
///                    those described in 'winborder'. If it is an array, it
///                    should have a length of eight or any divisor of eight. The
///                    array will specify the eight chars building up the border
///                    in a clockwise fashion starting with the top-left corner.
///                    As an example, the double box style could be specified as: >
///                     [ "╔", "═" ,"╗", "║", "╝", "═", "╚", "║" ].
///<
///                    If the number of chars are less than eight, they will be
///                    repeated. Thus an ASCII border could be specified as >
///                     [ "/", "-", \"\\\\\", "|" ],
///<
///                    or all chars the same as >
///                     [ "x" ].
///<
///                    An empty string can be used to turn off a specific border,
///                    for instance, >
///                     [ "", "", "", ">", "", "", "", "<" ]
///<
///                    will only make vertical borders but not horizontal ones.
///                    By default, `FloatBorder` highlight is used, which links
///                    to `WinSeparator` when not defined. It could also be
///                    specified by character: >
///                     [ ["+", "MyCorner"], ["x", "MyBorder"] ].
///<
///                  • title: Title (optional) in window border, string or list.
///                    List should consist of `[text, highlight]` tuples. If
///                    string, or a tuple lacks a highlight, the default
///                    highlight group is `FloatTitle`.
///                  • title_pos: Title position. Must be set with `title`
///                    option. Value can be one of "left", "center", or "right".
///                    Default is `"left"`.
///                  • footer: Footer (optional) in window border, string or
///                    list. List should consist of `[text, highlight]` tuples.
///                    If string, or a tuple lacks a highlight, the default
///                    highlight group is `FloatFooter`.
///                  • footer_pos: Footer position. Must be set with `footer`
///                    option. Value can be one of "left", "center", or "right".
///                    Default is `"left"`.
///                  • noautocmd: If true then all autocommands are blocked for
///                    the duration of the call.
///                  • fixed: If true when anchor is NW or SW, the float window
///                    would be kept fixed even if the window would be truncated.
///                  • hide: If true the floating window will be hidden.
///                  • vertical: Split vertically |:vertical|.
///                  • split: Split direction: "left", "right", "above", "below".
///
///    Return: ~
///        |window-ID|, or 0 on error
///
async fn open_win_wv(&self, buffer: Buffer, enter: Boolean, config: Dict, ) -> error::Result<Window> {
	self.call_fn_wv("nvim_open_win".into(), (buffer, enter, config, )).await
}
///nvim_win_set_config({window}, {config})                *nvim_win_set_config()*
///    Configures window layout. Cannot be used to move the last window in a
///    tabpage to a different one.
///
///    When reconfiguring a window, absent option keys will not be changed.
///    `row`/`col` and `relative` must be reconfigured together.
///
///    Attributes: ~
///        Since: 0.4.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///      • {config}  Map defining the window configuration, see |nvim_open_win()|
///
///    See also: ~
///      • |nvim_open_win()|
///
///
///==============================================================================
async fn win_set_config(&self, window: Window, config: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_win_set_config".into(), (window, config, )).await
}
///nvim_win_set_config({window}, {config})                *nvim_win_set_config()*
///    Configures window layout. Cannot be used to move the last window in a
///    tabpage to a different one.
///
///    When reconfiguring a window, absent option keys will not be changed.
///    `row`/`col` and `relative` must be reconfigured together.
///
///    Attributes: ~
///        Since: 0.4.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///      • {config}  Map defining the window configuration, see |nvim_open_win()|
///
///    See also: ~
///      • |nvim_open_win()|
///
///
///==============================================================================
async fn win_set_config_wv(&self, window: Window, config: Dict, ) -> error::Result<()> {
	self.call_fn_wv("nvim_win_set_config".into(), (window, config, )).await
}
///nvim_win_get_config({window})                          *nvim_win_get_config()*
///    Gets window configuration.
///
///    The returned value may be given to |nvim_open_win()|.
///
///    `relative` is empty for normal windows.
///
///    Attributes: ~
///        Since: 0.4.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///
///    Return: ~
///        Map defining the window configuration, see |nvim_open_win()|
///
async fn win_get_config<D: Deserialize<'static>>(&self, window: Window, ) -> error::Result<D> {
	self.call_fn("nvim_win_get_config".into(), (window, )).await
}
///nvim_win_get_config({window})                          *nvim_win_get_config()*
///    Gets window configuration.
///
///    The returned value may be given to |nvim_open_win()|.
///
///    `relative` is empty for normal windows.
///
///    Attributes: ~
///        Since: 0.4.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///
///    Return: ~
///        Map defining the window configuration, see |nvim_open_win()|
///
async fn win_get_config_wv(&self, window: Window, ) -> error::Result<Dict> {
	self.call_fn_wv("nvim_win_get_config".into(), (window, )).await
}
///nvim_win_get_buf({window})                                *nvim_win_get_buf()*
///    Gets the current buffer in a window
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///
///    Return: ~
///        Buffer id
///
async fn win_get_buf(&self, window: Window, ) -> error::Result<Buffer> {
	self.call_fn("nvim_win_get_buf".into(), (window, )).await
}
///nvim_win_set_buf({window}, {buffer})                      *nvim_win_set_buf()*
///    Sets the current buffer in a window, without side effects
///
///    Attributes: ~
///        not allowed when |textlock| is active
///        Since: 0.3.2
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///      • {buffer}  Buffer id
///
async fn win_set_buf(&self, window: Window, buffer: Buffer, ) -> error::Result<()> {
	self.call_fn("nvim_win_set_buf".into(), (window, buffer, )).await
}
///nvim_win_get_cursor({window})                          *nvim_win_get_cursor()*
///    Gets the (1,0)-indexed, buffer-relative cursor position for a given window
///    (different windows showing the same buffer have independent cursor
///    positions). |api-indexing|
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///
///    Return: ~
///        (row, col) tuple
///
///    See also: ~
///      • |getcurpos()|
///
async fn win_get_cursor(&self, window: Window, ) -> error::Result<Vec<Integer>> {
	self.call_fn("nvim_win_get_cursor".into(), (window, )).await
}
///nvim_win_set_cursor({window}, {pos})                   *nvim_win_set_cursor()*
///    Sets the (1,0)-indexed cursor position in the window. |api-indexing| This
///    scrolls the window even if it is not the current one.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///      • {pos}     (row, col) tuple representing the new position
///
async fn win_set_cursor(&self, window: Window, pos: &[Integer], ) -> error::Result<()> {
	self.call_fn("nvim_win_set_cursor".into(), (window, pos, )).await
}
///nvim_win_get_height({window})                          *nvim_win_get_height()*
///    Gets the window height
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///
///    Return: ~
///        Height as a count of rows
///
async fn win_get_height(&self, window: Window, ) -> error::Result<Integer> {
	self.call_fn("nvim_win_get_height".into(), (window, )).await
}
///nvim_win_set_height({window}, {height})                *nvim_win_set_height()*
///    Sets the window height.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///      • {height}  Height as a count of rows
///
async fn win_set_height(&self, window: Window, height: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_win_set_height".into(), (window, height, )).await
}
///nvim_win_get_width({window})                            *nvim_win_get_width()*
///    Gets the window width
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///
///    Return: ~
///        Width as a count of columns
///
async fn win_get_width(&self, window: Window, ) -> error::Result<Integer> {
	self.call_fn("nvim_win_get_width".into(), (window, )).await
}
///nvim_win_set_width({window}, {width})                   *nvim_win_set_width()*
///    Sets the window width. This will only succeed if the screen is split
///    vertically.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///      • {width}   Width as a count of columns
///
async fn win_set_width(&self, window: Window, width: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_win_set_width".into(), (window, width, )).await
}
///nvim_win_get_var({window}, {name})                        *nvim_win_get_var()*
///    Gets a window-scoped (w:) variable
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///      • {name}    Variable name
///
///    Return: ~
///        Variable value
///
async fn win_get_var<D: Deserialize<'static>>(&self, window: Window, name: &str, ) -> error::Result<D> {
	self.call_fn("nvim_win_get_var".into(), (window, name, )).await
}
///nvim_win_get_var({window}, {name})                        *nvim_win_get_var()*
///    Gets a window-scoped (w:) variable
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///      • {name}    Variable name
///
///    Return: ~
///        Variable value
///
async fn win_get_var_wv(&self, window: Window, name: String, ) -> error::Result<Object> {
	self.call_fn_wv("nvim_win_get_var".into(), (window, name, )).await
}
///nvim_win_set_var({window}, {name}, {value})               *nvim_win_set_var()*
///    Sets a window-scoped (w:) variable
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///      • {name}    Variable name
///      • {value}   Variable value
///
async fn win_set_var(&self, window: Window, name: &str, value: impl Serialize, ) -> error::Result<()> {
	self.call_fn("nvim_win_set_var".into(), (window, name, value, )).await
}
///nvim_win_set_var({window}, {name}, {value})               *nvim_win_set_var()*
///    Sets a window-scoped (w:) variable
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///      • {name}    Variable name
///      • {value}   Variable value
///
async fn win_set_var_wv(&self, window: Window, name: String, value: Object, ) -> error::Result<()> {
	self.call_fn_wv("nvim_win_set_var".into(), (window, name, value, )).await
}
///nvim_win_del_var({window}, {name})                        *nvim_win_del_var()*
///    Removes a window-scoped (w:) variable
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///      • {name}    Variable name
///
async fn win_del_var(&self, window: Window, name: &str, ) -> error::Result<()> {
	self.call_fn("nvim_win_del_var".into(), (window, name, )).await
}
///nvim_win_get_position({window})                      *nvim_win_get_position()*
///    Gets the window position in display cells. First position is zero.
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///
///    Return: ~
///        (row, col) tuple with the window position
///
async fn win_get_position(&self, window: Window, ) -> error::Result<Vec<Integer>> {
	self.call_fn("nvim_win_get_position".into(), (window, )).await
}
///nvim_win_get_tabpage({window})                        *nvim_win_get_tabpage()*
///    Gets the window tabpage
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///
///    Return: ~
///        Tabpage that contains the window
///
async fn win_get_tabpage(&self, window: Window, ) -> error::Result<Tabpage> {
	self.call_fn("nvim_win_get_tabpage".into(), (window, )).await
}
///nvim_win_get_number({window})                          *nvim_win_get_number()*
///    Gets the window number
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///
///    Return: ~
///        Window number
///
async fn win_get_number(&self, window: Window, ) -> error::Result<Integer> {
	self.call_fn("nvim_win_get_number".into(), (window, )).await
}
///nvim_win_is_valid({window})                              *nvim_win_is_valid()*
///    Checks if a window is valid
///
///    Attributes: ~
///        Since: 0.1.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///
///    Return: ~
///        true if the window is valid, false otherwise
///
async fn win_is_valid(&self, window: Window, ) -> error::Result<Boolean> {
	self.call_fn("nvim_win_is_valid".into(), (window, )).await
}
///nvim_win_hide({window})                                      *nvim_win_hide()*
///    Closes the window and hide the buffer it contains (like |:hide| with a
///    |window-ID|).
///
///    Like |:hide| the buffer becomes hidden unless another window is editing
///    it, or 'bufhidden' is `unload`, `delete` or `wipe` as opposed to |:close|
///    or |nvim_win_close()|, which will close the buffer.
///
///    Attributes: ~
///        not allowed when |textlock| is active
///        Since: 0.5.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///
async fn win_hide(&self, window: Window, ) -> error::Result<()> {
	self.call_fn("nvim_win_hide".into(), (window, )).await
}
///nvim_win_close({window}, {force})                           *nvim_win_close()*
///    Closes the window (like |:close| with a |window-ID|).
///
///    Attributes: ~
///        not allowed when |textlock| is active
///        Since: 0.4.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window
///      • {force}   Behave like `:close!` The last window of a buffer with
///                  unwritten changes can be closed. The buffer will become
///                  hidden, even if 'hidden' is not set.
///
async fn win_close(&self, window: Window, force: Boolean, ) -> error::Result<()> {
	self.call_fn("nvim_win_close".into(), (window, force, )).await
}
///nvim_win_set_hl_ns({window}, {ns_id})                   *nvim_win_set_hl_ns()*
///    Set highlight namespace for a window. This will use highlights defined
///    with |nvim_set_hl()| for this namespace, but fall back to global
///    highlights (ns=0) when missing.
///
///    This takes precedence over the 'winhighlight' option.
///
///    Attributes: ~
///        Since: 0.8.0
///
///    Parameters: ~
///      • {ns_id}   the namespace to use
///
async fn win_set_hl_ns(&self, window: Window, ns_id: Integer, ) -> error::Result<()> {
	self.call_fn("nvim_win_set_hl_ns".into(), (window, ns_id, )).await
}
///nvim_win_text_height({window}, {opts})                *nvim_win_text_height()*
///    Computes the number of screen lines occupied by a range of text in a given
///    window. Works for off-screen text and takes folds into account.
///
///    Diff filler or virtual lines above a line are counted as a part of that
///    line, unless the line is on "start_row" and "start_vcol" is specified.
///
///    Diff filler or virtual lines below the last buffer line are counted in the
///    result when "end_row" is omitted.
///
///    Line indexing is similar to |nvim_buf_get_text()|.
///
///    Attributes: ~
///        Since: 0.10.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window.
///      • {opts}    Optional parameters:
///                  • start_row: Starting line index, 0-based inclusive. When
///                    omitted start at the very top.
///                  • end_row: Ending line index, 0-based inclusive. When
///                    omitted end at the very bottom.
///                  • start_vcol: Starting virtual column index on "start_row",
///                    0-based inclusive, rounded down to full screen lines. When
///                    omitted include the whole line.
///                  • end_vcol: Ending virtual column index on "end_row",
///                    0-based exclusive, rounded up to full screen lines. When
///                    omitted include the whole line.
///
///    Return: ~
///        Dict containing text height information, with these keys:
///        • all: The total number of screen lines occupied by the range.
///        • fill: The number of diff filler or virtual lines among them.
///
///    See also: ~
///      • |virtcol()| for text width.
///
///
///==============================================================================
async fn win_text_height<D: Deserialize<'static>>(&self, window: Window, opts: impl Serialize, ) -> error::Result<D> {
	self.call_fn("nvim_win_text_height".into(), (window, opts, )).await
}
///nvim_win_text_height({window}, {opts})                *nvim_win_text_height()*
///    Computes the number of screen lines occupied by a range of text in a given
///    window. Works for off-screen text and takes folds into account.
///
///    Diff filler or virtual lines above a line are counted as a part of that
///    line, unless the line is on "start_row" and "start_vcol" is specified.
///
///    Diff filler or virtual lines below the last buffer line are counted in the
///    result when "end_row" is omitted.
///
///    Line indexing is similar to |nvim_buf_get_text()|.
///
///    Attributes: ~
///        Since: 0.10.0
///
///    Parameters: ~
///      • {window}  |window-ID|, or 0 for current window.
///      • {opts}    Optional parameters:
///                  • start_row: Starting line index, 0-based inclusive. When
///                    omitted start at the very top.
///                  • end_row: Ending line index, 0-based inclusive. When
///                    omitted end at the very bottom.
///                  • start_vcol: Starting virtual column index on "start_row",
///                    0-based inclusive, rounded down to full screen lines. When
///                    omitted include the whole line.
///                  • end_vcol: Ending virtual column index on "end_row",
///                    0-based exclusive, rounded up to full screen lines. When
///                    omitted include the whole line.
///
///    Return: ~
///        Dict containing text height information, with these keys:
///        • all: The total number of screen lines occupied by the range.
///        • fill: The number of diff filler or virtual lines among them.
///
///    See also: ~
///      • |virtcol()| for text width.
///
///
///==============================================================================
async fn win_text_height_wv(&self, window: Window, opts: Dict, ) -> error::Result<Dict> {
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
Self::Unknown(_) => "unknown"}}}
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
	let inner = Vec::<ModeInfoSet>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::ModeInfoSet(inner));
},
"update_menu" => {
	let inner = Vec::<UpdateMenu>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::UpdateMenu(inner));
},
"busy_start" => {
	let inner = Vec::<BusyStart>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::BusyStart(inner));
},
"busy_stop" => {
	let inner = Vec::<BusyStop>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::BusyStop(inner));
},
"mouse_on" => {
	let inner = Vec::<MouseOn>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::MouseOn(inner));
},
"mouse_off" => {
	let inner = Vec::<MouseOff>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::MouseOff(inner));
},
"mode_change" => {
	let inner = Vec::<ModeChange>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::ModeChange(inner));
},
"bell" => {
	let inner = Vec::<Bell>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::Bell(inner));
},
"visual_bell" => {
	let inner = Vec::<VisualBell>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::VisualBell(inner));
},
"flush" => {
	let inner = Vec::<Flush>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::Flush(inner));
},
"suspend" => {
	let inner = Vec::<Suspend>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::Suspend(inner));
},
"set_title" => {
	let inner = Vec::<SetTitle>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::SetTitle(inner));
},
"set_icon" => {
	let inner = Vec::<SetIcon>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::SetIcon(inner));
},
"screenshot" => {
	let inner = Vec::<Screenshot>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::Screenshot(inner));
},
"option_set" => {
	let inner = Vec::<OptionSet>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::OptionSet(inner));
},
"chdir" => {
	let inner = Vec::<Chdir>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::Chdir(inner));
},
"update_fg" => {
	let inner = Vec::<UpdateFg>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::UpdateFg(inner));
},
"update_bg" => {
	let inner = Vec::<UpdateBg>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::UpdateBg(inner));
},
"update_sp" => {
	let inner = Vec::<UpdateSp>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::UpdateSp(inner));
},
"resize" => {
	let inner = Vec::<Resize>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::Resize(inner));
},
"clear" => {
	let inner = Vec::<Clear>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::Clear(inner));
},
"eol_clear" => {
	let inner = Vec::<EolClear>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::EolClear(inner));
},
"cursor_goto" => {
	let inner = Vec::<CursorGoto>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::CursorGoto(inner));
},
"highlight_set" => {
	let inner = Vec::<HighlightSet>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::HighlightSet(inner));
},
"put" => {
	let inner = Vec::<Put>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::Put(inner));
},
"set_scroll_region" => {
	let inner = Vec::<SetScrollRegion>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::SetScrollRegion(inner));
},
"scroll" => {
	let inner = Vec::<Scroll>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::Scroll(inner));
},
"default_colors_set" => {
	let inner = Vec::<DefaultColorsSet>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::DefaultColorsSet(inner));
},
"hl_attr_define" => {
	let inner = Vec::<HlAttrDefine>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::HlAttrDefine(inner));
},
"hl_group_set" => {
	let inner = Vec::<HlGroupSet>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::HlGroupSet(inner));
},
"grid_resize" => {
	let inner = Vec::<GridResize>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::GridResize(inner));
},
"grid_clear" => {
	let inner = Vec::<GridClear>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::GridClear(inner));
},
"grid_cursor_goto" => {
	let inner = Vec::<GridCursorGoto>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::GridCursorGoto(inner));
},
"grid_line" => {
	let inner = Vec::<GridLine>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::GridLine(inner));
},
"grid_scroll" => {
	let inner = Vec::<GridScroll>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::GridScroll(inner));
},
"grid_destroy" => {
	let inner = Vec::<GridDestroy>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::GridDestroy(inner));
},
"win_pos" => {
	let inner = Vec::<WinPos>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WinPos(inner));
},
"win_float_pos" => {
	let inner = Vec::<WinFloatPos>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WinFloatPos(inner));
},
"win_external_pos" => {
	let inner = Vec::<WinExternalPos>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WinExternalPos(inner));
},
"win_hide" => {
	let inner = Vec::<WinHide>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WinHide(inner));
},
"win_close" => {
	let inner = Vec::<WinClose>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WinClose(inner));
},
"msg_set_pos" => {
	let inner = Vec::<MsgSetPos>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::MsgSetPos(inner));
},
"win_viewport" => {
	let inner = Vec::<WinViewport>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WinViewport(inner));
},
"win_viewport_margins" => {
	let inner = Vec::<WinViewportMargins>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WinViewportMargins(inner));
},
"win_extmark" => {
	let inner = Vec::<WinExtmark>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WinExtmark(inner));
},
"popupmenu_show" => {
	let inner = Vec::<PopupmenuShow>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::PopupmenuShow(inner));
},
"popupmenu_hide" => {
	let inner = Vec::<PopupmenuHide>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::PopupmenuHide(inner));
},
"popupmenu_select" => {
	let inner = Vec::<PopupmenuSelect>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::PopupmenuSelect(inner));
},
"tabline_update" => {
	let inner = Vec::<TablineUpdate>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::TablineUpdate(inner));
},
"cmdline_show" => {
	let inner = Vec::<CmdlineShow>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::CmdlineShow(inner));
},
"cmdline_pos" => {
	let inner = Vec::<CmdlinePos>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::CmdlinePos(inner));
},
"cmdline_special_char" => {
	let inner = Vec::<CmdlineSpecialChar>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::CmdlineSpecialChar(inner));
},
"cmdline_hide" => {
	let inner = Vec::<CmdlineHide>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::CmdlineHide(inner));
},
"cmdline_block_show" => {
	let inner = Vec::<CmdlineBlockShow>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::CmdlineBlockShow(inner));
},
"cmdline_block_append" => {
	let inner = Vec::<CmdlineBlockAppend>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::CmdlineBlockAppend(inner));
},
"cmdline_block_hide" => {
	let inner = Vec::<CmdlineBlockHide>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::CmdlineBlockHide(inner));
},
"wildmenu_show" => {
	let inner = Vec::<WildmenuShow>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WildmenuShow(inner));
},
"wildmenu_select" => {
	let inner = Vec::<WildmenuSelect>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WildmenuSelect(inner));
},
"wildmenu_hide" => {
	let inner = Vec::<WildmenuHide>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::WildmenuHide(inner));
},
"msg_show" => {
	let inner = Vec::<MsgShow>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::MsgShow(inner));
},
"msg_clear" => {
	let inner = Vec::<MsgClear>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::MsgClear(inner));
},
"msg_showcmd" => {
	let inner = Vec::<MsgShowcmd>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::MsgShowcmd(inner));
},
"msg_showmode" => {
	let inner = Vec::<MsgShowmode>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::MsgShowmode(inner));
},
"msg_ruler" => {
	let inner = Vec::<MsgRuler>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::MsgRuler(inner));
},
"msg_history_show" => {
	let inner = Vec::<MsgHistoryShow>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::MsgHistoryShow(inner));
},
"msg_history_clear" => {
	let inner = Vec::<MsgHistoryClear>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::MsgHistoryClear(inner));
},
"error_exit" => {
	let inner = Vec::<ErrorExit>::deserialize(ContSeq::new(seq))?;
	return Ok(UiEvent::ErrorExit(inner));
},

        o => {
            let inner = Value::deserialize(ContSeq::new(seq))?;
            return Ok(UiEvent::Unknown(Box::new((o.to_string(), inner))));
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
