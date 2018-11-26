# @param {String} str
# @return {Integer}
def my_atoi(str)
  num = str[/^\s*(\+|-)?(\d+)/]&.gsub!(/^\s*((\+|-)?)0*/, '\1')
  return 0 if num == nil || num.length == 0 || num == '+' || num == '-'
  num = Integer(num)
  if num >= (1 << 31)
    num = (1 << 31) - 1
  elsif num < -(1 << 31)
    num = -(1 << 31)
  end
  return num
end
