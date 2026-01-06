-- ============================================================================
-- EOK Configuration
-- Mod options and save/load functionality
-- ============================================================================

-- Get option with default fallback
local function get_option(key, default)
    if EXPOK and EXPOK._data and EXPOK._data[key] ~= nil then
        return EXPOK._data[key]
    end
    return default
end

-- Save config to file
local function save_config()
    if not EXPOK or not EXPOK._data_path then return end
    local file = io.open(EXPOK._data_path, "w+")
    if file then
        file:write(json.encode(EXPOK._data))
        file:close()
    end
end

-- Export functions
return {
    get_option = get_option,
    save_config = save_config
}
