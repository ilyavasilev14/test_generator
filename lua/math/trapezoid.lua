-- логика создания заданий пишется на языке программирования Lua
-- больше о создании интерфейсов заданий: https://github.com/ilyavasilev14/test_generator/blob/master/docs/custom_gui.md

-- генерация задания
local ex_type = math.random(0, 1)

if ex_type == 0 then
    -- неизвестна ср. линия
    local side1
    local side2
    local midline
    while true do
        side1 = math.random(10, 50)
        side2 = math.random(side1 + 5, 70)
        midline = (side1 + side2) / 2
        if math.floor(midline) == midline then
            goto ex_type_0_ready
        end
    end
    ::ex_type_0_ready::
    text = "Основания трапеции равны " .. side1 .. " и " .. side2
        .. ". Найдите длину средней линии трапеции."
    answer = tostring(math.floor(midline))
else
    -- неизвестно основание
    local side1
    local side2
    local midline
    while true do
        side1 = math.random(10, 50)
        midline = math.random(side1 + 5, 70)
        side2 = 2 * midline - side1
        if math.floor(side2) == side2 then
            goto ex_type_1_ready
        end
    end
    ::ex_type_1_ready::
    text = "Первое основание трапеции равно " .. side1 .. ", длина средней линии равна " .. midline
        .. ". Найдите второе основание трапеции."
    answer = tostring(math.floor(side2))
end

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
    return "Средняя линия трапеции"
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
