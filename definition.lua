---@meta

---@class Game
---@field window Window
---@field color Color
---@field inner_events table inner game event name
game = {}

---@return GameObject
---@param table table
function game:instantiate(table) end

---@param object GameObject
---@param func string
function game:hook(object, func) end

---@param func string
---@param ... any
function game:call(func, ...) end

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

---@param position IVector2
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
---@field r uint number in range [0; 255]
---@field g uint number in range [0; 255]
---@field b uint number in range [0; 255]
---@field a uint number in range [0; 255]
color = {}

---@return Color
---@param r uint number in range [0; 255]
---@param g uint number in range [0; 255]
---@param b uint number in range [0; 255]
---@param a uint number in range [0; 255]
function color:rgba(r, g, b, a) end

---@return Color Color with rgb and a: 255
---@param r uint number in range [0; 255]
---@param g uint number in range [0; 255]
---@param b uint number in range [0; 255]
function color:rgb(r, g, b) end

---@return Color
function color:invert() end

---@return Color Color with r: 255 g:255 b:255 a:255
function color:WHITE() end

---@return Color Color with r:0 g:0 b:0 a:255
function color:BLACK() end

---@return Color Color with r:128 g:128 b:128 a:255
function color:GREY() end

---@return Color Color with r:255 g:0 b:0 a:255
function color:RED() end

---@return Color Color with r:0 g:255 b:0 a:255
function color:GREEN() end

---@return Color Color with r:0 g:0 b:255 a:255
function color:BLUE() end

---@return Color Color with r:255 g:0 b:255 a:255
function color:MAGENTA() end

---@return Color Color with r:255 g:255 b:0 a:255
function color:YELLOW() end

---@return Color Color with r:0 g:255 b:255 a:255
function color:CYAN() end

---@package
---@class Size
---@field width number
---@field height number
size = {}

---@package
---@class GameObject
__gameobject = {}

---@package
---@class float: number
__float = {}

---@package
---@class int: number
__int = {}

---@package
---@class uint: number
__uint = {}

---@class IVector2: Vector
---@field x int
---@field y int
_ = {}

---@class FVector2: Vector
---@field x float
---@field y float
_ = {}

---@class IVector3: Vector
---@field x int
---@field y int
---@field z int
_ = {}

---@class FVector3: Vector
---@field x float
---@field y float
---@field z float
_ = {}

---@class IVector4: Vector
---@field x int
---@field y int
---@field z int
---@field w int
_ = {}

---@class FVector4: Vector
---@field x float
---@field y float
---@field z float
---@field w float
_ = {}

---@class Vector
vector = {}

---@return IVector2 IVector2 if number integer
---@return FVector2 FVector2 if number float
---@param x number
---@param y number
function vector:new2d(x, y) end

---@return IVector3 IVector3 if number integer
---@return FVector3 FVector3 if number float
---@param x number
---@param y number
---@param z number
function vector:new3d(x, y, z) end

---@return IVector4 IVector4 if number integer
---@return FVector4 FVector4 if number float
---@param x number
---@param y number
---@param z number
---@param w number
function vector:new4d(x, y, z, w) end
