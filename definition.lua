---@meta

---@class Game
---@field window Window
---@field inner_events table inner game event name
game = {}

---@return GameObject
---@param table table
function game:instantiate(table) end

---@param object GameObject
---@param func string
function game:subscribe(object, func) end

---@param func string
---@param ... any
function game:emit(func, ...) end

---@class Window
---@field size Size
---@field is_always_on_top boolean
---@field is_maximized boolean
---@field is_minimized boolean
window = {}

---@param width uint
---@param height uint
function window:set_size(width, height) end

---@param color Color
function window:set_color(color) end

---@param position Vector2
function window:set_position(position) end

---@param resizable boolean
function window:set_resizable(resizable) end

---@param bordered boolean
function window:set_bordered(bordered) end

---@param flag boolean
function window:set_fullscreen(flag) end

---@param title string
function window:set_title(title) end

---@deprecated
---@param icon nil
function window:set_icon(icon) end

function window:show() end
function window:hide() end
function window:raise() end
function window:maximize() end
function window:minimize() end
function window:restore() end

---@class Color
---@field r subpixel
---@field g subpixel
---@field b subpixel
---@field a subpixel
color = {}

---
--- Create color with fields [r, g, b, a]
---
---@return Color
---@param r subpixel
---@param g subpixel
---@param b subpixel
---@param a subpixel
function color.rgba(r, g, b, a) end

---
--- Create color with fields [r, g, b, a: 255]
---
---@return Color
---@param r subpixel
---@param g subpixel
---@param b subpixel
function color.rgb(r, g, b) end

---
--- Transform number in subpixel [0; 255]
---
---@return subpixel
---@param n number
function color.subpixel(n) end

---@return Color
---@param color Color
function color.invert(color) end

---@return Color Color with r: 255 g:255 b:255 a:255
function color.white() end

---@return Color Color with r:0 g:0 b:0 a:255
function color.black() end

---@return Color Color with r:128 g:128 b:128 a:255
function color.grey() end

---@return Color Color with r:255 g:0 b:0 a:255
function color.red() end

---@return Color Color with r:0 g:255 b:0 a:255
function color.green() end

---@return Color Color with r:0 g:0 b:255 a:255
function color.blue() end

---@return Color Color with r:255 g:0 b:255 a:255
function color.magenta() end

---@return Color Color with r:255 g:255 b:0 a:255
function color.yellow() end

---@return Color Color with r:0 g:255 b:255 a:255
function color.cyan() end

---@package
---@class Size
---@field width number
---@field height number
_ = {}

---@package
---@class GameObject
_ = {}

---
--- Subpixel is number in range [0; 255]
---
---@package
---@class subpixel: number
_ = {}

---@package
---@class Vector2: Vector
---@field x number
---@field y number
_ = {}

---@package
---@class Vector3: Vector
---@field x number
---@field y number
---@field z number
_ = {}

---@package
---@class Vector4: Vector
---@field x number
---@field y number
---@field z number
---@field w number
_ = {}

---@package
---@class Vector
vector = {}

---@return Vector2
---@param x number
---@param y number
function vector.new2d(x, y) end

---@return Vector3
---@param x number
---@param y number
---@param z number
function vector.new3d(x, y, z) end

---@return Vector4
---@param x number
---@param y number
---@param z number
---@param w number
function vector.new4d(x, y, z, w) end

---@class tracelib
---@field debug_mode boolean
tracing = {}

---
--- Print message with level ERROR
---
---@param ... any
function tracing.error(...) end

---
--- Print message with level WARN
---
---@param ... any
function tracing.warn(...) end

---
--- Print message with level INFO
---
---@see print is equal to this function
---@param ... any
function tracing.info(...) end

---
--- Print message only in debug mode
---
---@param ... any
function tracing.debug(...) end

---@class math: mathlib
---@field vector Vector
---@field color Color
math = {}
