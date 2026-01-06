-- ============================================================================
-- Menu Builder DSL
-- Generates BLT JSON menu definitions programmatically
-- ============================================================================

-- MenuBuilder class
local MenuBuilder = {}
MenuBuilder.__index = MenuBuilder

function MenuBuilder:new(title, id)
    local this = {
        config = {
            menu_id = id or "odrill_menu",
            parent_menu_id = "blt_options",
            title = title,
            description = title .. "_desc",
            items = {}
        },
        callbacks = {}
    }
    setmetatable(this, MenuBuilder)
    return this
end

function MenuBuilder:toggle(id, title_id, default_value, config_key)
    table.insert(self.config.items, {
        type = "toggle",
        id = id,
        title = title_id,
        description = title_id .. "_desc",
        callback = "callback_" .. id,
        value = config_key or id,
        default_value = default_value == nil and true or default_value
    })
    return self
end

function MenuBuilder:slider(id, title_id, default, min, max, step, config_key)
    table.insert(self.config.items, {
        type = "slider",
        id = id,
        title = title_id,
        description = title_id .. "_desc",
        callback = "callback_" .. id,
        value = config_key or id,
        default_value = default or 0,
        min = min or 0,
        max = max or 10,
        step = step or 1
    })
    return self
end

function MenuBuilder:multiple_choice(id, title_id, default, items, config_key)
    table.insert(self.config.items, {
        type = "multiple_choice",
        id = id,
        title = title_id,
        description = title_id .. "_desc",
        callback = "callback_" .. id,
        items = items,
        value = config_key or id,
        default_value = default or 1
    })
    return self
end

function MenuBuilder:divider(size)
    table.insert(self.config.items, {
        type = "divider",
        size = size or 10
    })
    return self
end

-- Builds the menu by registering it with BLT
-- Since BLT usually wants a file, we write a temp file or use hook
function MenuBuilder:build(callback_handler)
    -- Register callbacks automatically? 
    -- User still needs to define callbacks in MenuCallbackHandler
    
    -- Save to temporary file in SavePath to be loaded by BLT
    local file_path = SavePath .. self.config.menu_id .. "_generated.txt"
    local file = io.open(file_path, "w+")
    if file then
        file:write(json.encode(self.config))
        file:close()
        
        -- Load it
        MenuHelper:LoadFromJsonFile(file_path, callback_handler, EXPOK._data)
    else
        log("[EOK] Failed to write menu config to " .. file_path)
    end
end


