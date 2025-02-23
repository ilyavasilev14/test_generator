-- логика создания заданий пишется на языке программирования Lua
-- больше о создании интерфейсов заданий: https://github.com/ilyavasilev14/test_generator/blob/master/docs/custom_gui.md

function right_pad(str, len)
    local result = str
    while #result < len do
        print(#result)
        result = result .. " "
    end
    return result
end

-- генерация задания
local s1 = math.random(90, 300)
local s2 = math.random(90, 300)
local s3 = math.random(90, 300)
local a = s1 + s2
local b = s2 + s3
local a_or_b = s1 + s2 + s3
local a_and_b = s2

text = "В языке запросов поискового сервера для обозначения логической операции «ИЛИ» используется символ «|», а для обозначения логической операции «И» - символ «&»."
    .. "\nВ таблице приведены запросы и количество найденных по ним страниц некоторого сегмента сети Интернет."
    .. "\nКакое количество страниц (в тысячах) будет найдено по запросу \"A & B?\""
    .. "\n\nЗапрос  Найдено страниц (в тысячах)" .. "\n" .. right_pad("A | B", 14) .. a_or_b
    .. "\n" .. right_pad("A", 16) .. a .. "\n" .. right_pad("B", 16) .. b


-- слеующая функция возвращает текст задания
function get_exercise_text()
    return text
end

-- слеующая функция возвращает правильный ответ на задание
function get_exercise_right_answer()
    return a_and_b
end

-- слеующая функция возвращает название генератора заданий
function get_exercise_title()
    return "№8 ОГЭ Информатика"
end

--[[ свой интерфейс для задания, опционально
function get_custom_gui(gui)
    gui:vertical(function (vertical_gui)
        vertical_gui:label("текст!", 72)
        vertical_gui:horizontal(function (horizontal_gui)
            horizontal_gui:button("кнопка 1", 20, {500.0, 100})
            horizontal_gui:button("кнопка 2", 23, {200.0, 100})
        end)
    end)
end]]--

--[[ своя логика проверки задания, опционально
function check_exercise(answer)
end
]]--
