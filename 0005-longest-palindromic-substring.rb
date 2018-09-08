def get_palindrome(s, l, r)
  return '' if l < 0 || r >= s.length
  return '' if s[l] != s[r]
  while l > 0 && r < s.length - 1
    return s[l, r - l + 1] if s[l - 1] != s[r + 1]
    l -= 1; r += 1
  end
  return s[l, r - l + 1]
end

# @param {String} s
# @return {String}
def longest_palindrome(s)
  result = ''
  s.each_char.with_index{ |c,i|
    result = [
      result,
      get_palindrome(s, i, i),
      get_palindrome(s, i, i + 1),
    ].max{|x,y| x.length - y.length}
  }
  return result;
end
