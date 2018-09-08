# @param {String} s
# @param {Integer} num_rows
# @return {String}
def convert(s, num_rows)
  return s if num_rows == 1
  n = num_rows
  result = ''
  loop_length = (1.0 * s.length / (2 * n - 2)).ceil
   (0..n-1).each {|j| (0..loop_length).each {|i|
    index1 = 2 * i * (n - 1) + j
    index2 = 2 * (i + 1) * (n - 1) - j
    result += s[index1] || ''
    result += s[index2] || '' if j != 0 && index1 != index2
  }}
  return result
end
