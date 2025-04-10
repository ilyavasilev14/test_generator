-- логика создания заданий пишется на языке программирования Lua
-- больше о создании интерфейсов заданий: https://github.com/ilyavasilev14/test_generator/blob/master/docs/custom_gui.md

-- генерация задания
local ex_type = math.random(0, 1)

if ex_type == 0 then
    -- неизвестна гипотенуза
    local leg1
    local leg2
    local hypotenuse
    while true do
        leg1 = math.random(10, 50)
        leg2 = math.random(10, 50)
        print((leg1*leg1) + (leg2*leg2))
        hypotenuse = math.sqrt((leg1*leg1) + (leg2*leg2))
        print(hypotenuse)
        if math.floor(hypotenuse) == hypotenuse then
            goto ex_type_0_ready
        end
    end
    ::ex_type_0_ready::
    text = "В треугольнике ABC угол C прямой. AC = " .. leg1 .. ", CB = " .. leg2
        .. ".\nНайдите длину стороны AB."
    answer = tostring(math.floor(hypotenuse))
else
    -- неизвестен катет
    local leg1
    local leg2
    local hypotenuse
    while true do
        leg1 = math.random(10, 50)
        hypotenuse = math.random(leg1 + 5, 70)
        leg2 = math.sqrt((hypotenuse*hypotenuse) - (leg1*leg1))
        print(leg2)
        if math.floor(leg2) == leg2 then
            goto ex_type_1_ready
        end
    end
    ::ex_type_1_ready::
    text = "В треугольнике ABC угол C прямой. AC = " .. leg1 .. ", AB = " .. hypotenuse
        .. ".\nНайдите длину стороны CB."
    answer = tostring(math.floor(leg2))
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
    return "Теорема Пифагора"
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
