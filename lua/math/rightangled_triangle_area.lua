-- логика создания заданий пишется на языке программирования Lua
-- больше о создании интерфейсов заданий: https://github.com/ilyavasilev14/test_generator/blob/master/docs/custom_gui.md

-- генерация задания
local ex_type = math.random(0, 1)

if ex_type == 0 then
    local leg1
    local leg2
    local area
    while true do
        leg1 = math.random(10, 50)
        leg2 = math.random(10, 50)
        area = (leg1*leg2)/2
        if math.floor(area) == area then
            goto ex_type_0_ready
        end
    end
    ::ex_type_0_ready::
    text = "В треугольнике ABC угол C прямой. AC = " .. leg1 .. ", CB = " .. leg2
        .. ".\nНайдите площадь треугольника ABC."
    answer = tostring(math.floor(area))
else
    local leg1
    local hypotenuse
    local area
    while true do
        ::regenerate_ex_type_1::
        leg1 = math.random(10, 50)
        hypotenuse = math.random(leg1 + 5, 70)
        local leg2 = math.sqrt((hypotenuse*hypotenuse) - (leg1*leg1))
        if math.floor(leg2) ~= leg2 then
            goto regenerate_ex_type_1
        end
        area = (leg1*leg2)/2
        if math.floor(area) == area then
            goto ex_type_1_ready
        end
    end
    ::ex_type_1_ready::

    text = "В треугольнике ABC угол C прямой. AC = " .. leg1 .. ", AB = " .. hypotenuse
        .. ".\nНайдите площадь треугольника ABC."
    answer = tostring(math.floor(area))
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
    return "Площадь прямоугольного треугольника"
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
