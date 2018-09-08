# @param {String} s
# @return {Integer}
def length_of_longest_substring(s)
    return 0 if s.length == 0

    max_length = 0
    char_index = []
    l = 0
    s.each_char.with_index { |c,i|
        if char_index[c.ord] && char_index[c.ord] >= l
          l = char_index[c.ord] + 1
        end
        char_index[c.ord] = i
        if i - l + 1 > max_length
          max_length = i - l + 1
        end
    }
    return max_length
end
