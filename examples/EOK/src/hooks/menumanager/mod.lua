-- ============================================================================
-- EOK MenuManager Hook
-- Initializes global config and registers menu
-- ============================================================================

use mods::config::get_option

-- ============================================================================
-- GLOBAL INITIALIZATION
-- ============================================================================

-- Initialize EXPOK global table with defaults
_G.EXPOK = _G.EXPOK or {}
EXPOK._path = ModPath
EXPOK._data_path = SavePath .. "EXPOK_data.txt"
EXPOK._data = {
    exp_text_duration = 5,
    kln_duration = 2,
    show_announcer = true,
    exp_font_size = 15,
    kln_font_size = 15,
    exp_adjust_x = 0,
    exp_adjust_y = 0,
    kln_adjust_x = 0,
    kln_adjust_y = 0,
    gain_exp_on_kills = true,
    shortcut_choice_exp = 2,
    exp_fade_out_style = 1,
    exp_color = 14,
    specialkilled_color = 2,
    disable_hud_effects = false,
    anim_choice_style = 1  -- 1 = Classic, 2 = Modern
}

-- Helper functions
local function round(num, decimals)
    decimals = math.pow(10, decimals or 0)
    num = num * decimals
    if num >= 0 then
        num = math.floor(num + 0.5)
    else
        num = math.ceil(num - 0.5)
    end
    return num / decimals
end

function EXPOK:Save()
    local file = io.open(self._data_path, "w+")
    if file then
        file:write(json.encode(self._data))
        file:close()
    end
end

function EXPOK:Load()
    local file = io.open(self._data_path, "r")
    if file then
        local data = json.decode(file:read("*all"))
        if data then
            self._data = data
        end
        file:close()
    end
end

-- ============================================================================
-- MENU CALLBACKS
-- ============================================================================

Hooks:Add("MenuManagerInitialize", "EOK_MenuManagerInitialize", function(menu_manager)
    EXPOK:Load()
    
    -- Toggle callbacks
    MenuCallbackHandler.callback_EXPOK_show_announcer = function(self, item)
        EXPOK._data.show_announcer = item:value() == "on"
        EXPOK:Save()
    end
    
    MenuCallbackHandler.callback_EXPOK_disable_hud_effects = function(self, item)
        EXPOK._data.disable_hud_effects = item:value() == "on"
        EXPOK:Save()
    end
    
    MenuCallbackHandler.callback_EXPOK_gain_exp_on_kills = function(self, item)
        EXPOK._data.gain_exp_on_kills = item:value() == "on"
        EXPOK:Save()
    end
    
    -- Style choice callback
    MenuCallbackHandler.anim_choice_style_exp_callback = function(self, item)
        EXPOK._data.anim_choice_style = item:value()
        EXPOK:Save()
    end
    
    -- Slider callbacks
    MenuCallbackHandler.exp_text_duration_change_callback = function(self, item)
        EXPOK._data.exp_text_duration = item:value()
        EXPOK:Save()
    end
    
    MenuCallbackHandler.kln_duration_change_callback = function(self, item)
        EXPOK._data.kln_duration = item:value()
        EXPOK:Save()
    end
    
    MenuCallbackHandler.exp_font_size_change_callback = function(self, item)
        EXPOK._data.exp_font_size = round(item:value())
        EXPOK:Save()
    end
    
    MenuCallbackHandler.kln_font_size_change_callback = function(self, item)
        EXPOK._data.kln_font_size = round(item:value())
        EXPOK:Save()
    end
    
    MenuCallbackHandler.exp_adjust_x_change_callback = function(self, item)
        EXPOK._data.exp_adjust_x = item:value()
        EXPOK:Save()
    end
    
    MenuCallbackHandler.exp_adjust_y_change_callback = function(self, item)
        EXPOK._data.exp_adjust_y = item:value()
        EXPOK:Save()
    end
    
    MenuCallbackHandler.kln_adjust_x_change_callback = function(self, item)
        EXPOK._data.kln_adjust_x = item:value()
        EXPOK:Save()
    end
    
    MenuCallbackHandler.kln_adjust_y_change_callback = function(self, item)
        EXPOK._data.kln_adjust_y = item:value()
        EXPOK:Save()
    end
    
    -- Multiple choice callbacks
    MenuCallbackHandler.shortcut_choice_exp_callback = function(self, item)
        EXPOK._data.shortcut_choice_exp = item:value()
        EXPOK:Save()
    end
    
    MenuCallbackHandler.exp_fade_out_style_callback = function(self, item)
        EXPOK._data.exp_fade_out_style = item:value()
        EXPOK:Save()
    end
    
    MenuCallbackHandler.exp_color_change_callback = function(self, item)
        EXPOK._data.exp_color = item:value()
        EXPOK:Save()
    end
    
    MenuCallbackHandler.specialkilled_color_change_callback = function(self, item)
        EXPOK._data.specialkilled_color = item:value()
        EXPOK:Save()
    end
    
    MenuCallbackHandler.EXPOK_closed = function(self)
        EXPOK:Save()
    end
    
    -- Load menu from file if it exists
    if io.file_is_readable(EXPOK._path .. "menu/options.txt") then
        MenuHelper:LoadFromJsonFile(EXPOK._path .. "menu/options.txt", EXPOK, EXPOK._data)
    end
end)
