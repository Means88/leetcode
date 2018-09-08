# @param {Integer} x
# @return {Integer}
def reverse(x)
  sign = ''
  x = String(x)
  if x[0] == '-'
    sign = x[0]
    x = x[1..-1]
  end
  x = x.reverse
  while x.length > 1 && x[0] == '0'
    x = x[1..-1]
  end
  x = Integer("#{sign}#{x}")
  return 0 if x > (1 << 31) - 1 || x < -(1 << 31)
  return x
end
