# @param {String[]} strs
# @return {String}
def longest_common_prefix(strs)
  i = 0
  result = ''
  while true
    t = ''
    strs.each{|str|
      if t != '' && str[i] != t
        return result
      end
      t = str[i]
    }
    result += t
  end
end
