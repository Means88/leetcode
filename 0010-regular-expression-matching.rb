# @param {String} s
# @param {String} p
# @return {Boolean}
def is_match(s, p)
  return s.match("^#{p}$") != nil
end
