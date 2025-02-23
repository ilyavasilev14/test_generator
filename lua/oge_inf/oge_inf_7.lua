-- логика создания заданий пишется на языке программирования Lua
-- больше о создании интерфейсов заданий: https://github.com/ilyavasilev14/test_generator/blob/master/docs/custom_gui.md

-- генерация задания
local symbols_list = {"A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
    "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X",
    "Y", "Z", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0"
}

local server_name = ""
local server_name2 = "."
local address = ""

for _ = 1, math.random(4, 7) do
    server_name = server_name .. symbols_list[math.random(1, #symbols_list)]
end
for _ = 1, math.random(2, 4) do
    server_name2 = server_name2 .. symbols_list[math.random(1, #symbols_list)]
end
for _ = 1, math.random(2, 8) do
    address = address .. symbols_list[math.random(1, #symbols_list)]
end
local options = {address, "@", server_name, server_name2}
for _ = 1, math.random(1, 5) do
    local idx1 = math.random(1, #options)
    local idx2 = math.random(1, #options)

    local temp = options[idx1]
    options[idx1] = options[idx2]
    options[idx2] = temp
end

local answer_idx_list = {0, 0, 0, 0}
for idx,option in pairs(options) do
    if option == address then
        answer_idx_list[1] = idx
    elseif option == "@" then
        answer_idx_list[2] = idx
    elseif option == server_name then
        answer_idx_list[3] = idx
    elseif option == server_name2 then
        answer_idx_list[4] = idx
    end
end

local answer = ""
for _,idx in pairs(answer_idx_list) do
    if idx == 1 then
        answer = answer .. "А"
    elseif idx == 2 then
        answer = answer .. "Б"
    elseif idx == 3 then
        answer = answer .. "В"
    elseif idx == 4 then
        answer = answer .. "Г"
    end
end


text = "На сервере " .. server_name .. server_name2 .. " находится почтовый "
    .. "ящик " .. address .. ". Фрагменты адреса электронной почты закодированы буквами "
    .. "от А до Г. Запишите последовательность буквами от А до Г. Запишите последовательность "
    .. "букв, кодирующую этот адрес.\nА) " .. options[1] .. "\nБ) " .. options[2] .. "\nВ) "
    .. options[3] .. "\nГ) " .. options[4]

-- слеующая функция возвращает текст задания
function get_exercise_text()
    return text
end

-- слеующая функция возвращает правильный ответ на задание
function get_exercise_right_answer()
    return answer
end

-- слеующая функция возвращает название генератора заданий
function get_exercise_title()
    return "ОГЭ Информатика 7"
end

--свой интерфейс для задания, опционально
--[[function get_custom_gui(gui)
    gui:vertical(function (vertical_gui)
        vertical_gui:label("текст!", 72)
        vertical_gui:horizontal(function (horizontal_gui)
            horizontal_gui:button("кнопка 1", 20, {500.0, 100})
            horizontal_gui:button("кнопка 2", 23, {200.0, 100})
        end)
    end)
end--]]

--[[ своя логика проверки задания, опционально
function check_exercise(answer)
end
]]--
