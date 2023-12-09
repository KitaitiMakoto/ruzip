require_relative "helper"

class RuZipTest < Test::Unit::TestCase
  test "VERSION" do
    assert do
      ::RuZip.const_defined?(:VERSION)
    end
  end
end
