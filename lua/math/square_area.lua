-- логика создания заданий пишется на языке программирования Lua
-- больше о создании интерфейсов заданий: https://github.com/ilyavasilev14/test_generator/blob/master/docs/custom_gui.md

-- генерация задания
local ex_type = math.random(0, 1)

if ex_type == 0 then
    -- извествен радиус вписанной окружности
    local rad = math.random(10, 50)
    text = "Найдите площадь прямоугольника, описанного вокруг окружности с радиусом "
        .. rad .. "."
    answer = tostring(math.floor((rad*2)*(rad*2)))
else
    -- известна диагональ
    local area
    local diagonal
    while true do
        diagonal = math.random(1, 100)
        area = (diagonal*diagonal) / 2

        if math.floor(area) == area then
            goto ex_type_1_ready
        end
    end
    ::ex_type_1_ready::
    text = "Найдите площадь квадрата, диагональ которого равна " .. diagonal .. "."
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
    return "Площадь квадрата через радиус вписанной окружности, длину диагонали"
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
