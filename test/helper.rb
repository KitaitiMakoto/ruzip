$LOAD_PATH.unshift File.expand_path("../lib", __dir__)

require "ruzip"
require "test-unit"
require "test/unit/notify"

class Test::Unit::TestCase
  def fixture_path(component)
    File.join(__dir__, "fixtures", component)
  end
end
