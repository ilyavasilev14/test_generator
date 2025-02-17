local text
local right_answer

-- generating the exercise
local s_str
local t_str
local yes_counter = 0
local no_counter = 0

local s_sets = {}
local t_sets = {}

for _ = 1, 9 do
    table.insert(s_sets, math.random(2, 50))
    table.insert(t_sets, math.random(2, 50))
end

local exercise_type = math.random(1, 4)
if exercise_type == 1 then
    local s = math.random(2, 20)
    local t = math.random(s + 5, 40)
    s_str = "s > " .. s
    t_str = "t < " .. t
    for i = 1, 9 do
        if s_sets[i] > s or t_sets[i] < t then
            yes_counter = yes_counter + 1
        else
            no_counter = no_counter + 1
        end
    end
elseif exercise_type == 2 then
    local s = math.random(2, 20)
    local t = math.random(s + 5, 40)
    s_str = "s >= " .. s
    t_str = "t < " .. t
    for i = 1, 9 do
        if s_sets[i] >= s or t_sets[i] < t then
            yes_counter = yes_counter + 1
        else
            no_counter = no_counter + 1
        end
    end
elseif exercise_type == 3 then
    local s = math.random(10, 30)
    local t = s + math.random(5, s - 3)
    s_str = "s < " .. s
    t_str = "t > " .. t
    for i = 1, 9 do
        if s_sets[i] < s or t_sets[i] > t then
            yes_counter = yes_counter + 1
        else
            no_counter = no_counter + 1
        end
    end
elseif exercise_type == 4 then
    local s = math.random(10, 30)
    local t = s + math.random(5, s - 3)
    s_str = "s <= " .. s
    t_str = "t >= " .. t
    for i = 1, 9 do
        if s_sets[i] <= s or t_sets[i] >= t then
            yes_counter = yes_counter + 1
        else
            no_counter = no_counter + 1
        end
    end
end

local number_sets_string = ""
for i = 1, 9 do
    number_sets_string = number_sets_string .. "("
        .. s_sets[i] .. ", " .. t_sets[i] .. "); "
end

-- text here
local program_text = [[алг
нач
цел s, t
ввод s
ввод t
если ]] .. s_str .. [[ или ]] .. t_str
.. [[

    то вывод "YES"
    иначе вывод "NO"
все
кон]]

local is_right_answer_yes_count = math.random(0, 1)
local requested_output = ""
if is_right_answer_yes_count == 0 then
    right_answer = tostring(yes_counter)
    requested_output = '"YES"'
else
    right_answer = tostring(no_counter)
    requested_output = '"NO"'
end

text =
[[Ниже приведена программа, записанная на одном из языков программирования.
Алгоритмический язык:
]] .. program_text .. [[

Было проведено 9 запусков программы, при которых в качестве значений переменных s и t вводились следующие пары чисел:
]] .. number_sets_string ..
[[
Сколько было запусков, при которых программа напечатала ]] .. requested_output

function get_exercise_right_answer()
    return right_answer
end

function get_exercise_text()
    return text
end

function get_exercise_title()
    return "№6 ОГЭ информатика"
end

