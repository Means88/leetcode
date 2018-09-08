# @param {Integer} x
# @return {Boolean}
def is_palindrome(x)
  x = String(x)
  l = 0; r = x.length - 1
  while x[l] == x[r] && l < r
    l += 1; r -= 1
  end
  return x[l] == x[r]
end
