-- ============================================================================
-- Odrill Colors Library
-- Color utilities and presets for Payday 2
-- Author: Odrill Official Team
-- ============================================================================

-- Base color palette (RGB values 0-255)
local PALETTE = {
    RED = {255, 0, 0},
    GREEN = {0, 255, 0},
    BLUE = {0, 0, 255},
    WHITE = {255, 255, 255},
    BLACK = {0, 0, 0},
    YELLOW = {255, 255, 0},
    CYAN = {0, 255, 255},
    MAGENTA = {255, 0, 255},
    ORANGE = {255, 165, 0},
    PURPLE = {128, 0, 128},
    PINK = {255, 192, 203},
    AQUA = {0, 255, 221},
    NAVY = {0, 0, 128},
    LIME = {0, 255, 0},
    GOLD = {255, 215, 0},
    SILVER = {192, 192, 192},
    PASTEL_PINK = {255, 161, 220},
    LILAC = {223, 168, 255},
    STRAWBERRY = {251, 41, 65},
    BLUE_VIOLET = {169, 48, 255},
    PALE_YELLOW = {255, 243, 138},
}

--- Get a rainbow color cycling through RGB
--- @return number, number, number RGB values (0-1)
local function rainbow()
    local t = Application:time()
    local h = (t % 5) / 5 * 6
    local i = math.floor(h)
    local f = h - i
    local q = 1 - f
    local sector = i % 6
    
    if sector == 0 then return 1, f, 0
    elseif sector == 1 then return q, 1, 0
    elseif sector == 2 then return 0, 1, f
    elseif sector == 3 then return 0, q, 1
    elseif sector == 4 then return f, 0, 1
    else return 1, 0, q end
end

--- Get a random color from the palette
--- @return table RGB values {r, g, b}
local function random()
    local keys = {}
    for k in pairs(PALETTE) do
        table.insert(keys, k)
    end
    return PALETTE[keys[math.random(#keys)]]
end

--- Get color from palette by name
--- @param name string Color name (e.g., "RED", "BLUE")
--- @return table|nil RGB values or nil if not found
local function from_name(name)
    return PALETTE[name]
end

--- Convert palette color to normalized (0-1) values
--- @param color table RGB values {r, g, b} (0-255)
--- @return number, number, number RGB values (0-1)
local function normalize(color)
    return color[1] / 255, color[2] / 255, color[3] / 255
end

--- Create a Color object from palette
--- @param name string Color name
--- @return Color PD2 Color object
local function to_color(name)
    local c = PALETTE[name]
    if c then
        return Color(c[1] / 255, c[2] / 255, c[3] / 255)
    end
    return Color.white
end
