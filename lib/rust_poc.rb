# frozen_string_literal: true

require "rust_poc/version"
require "rust_poc/ffi"
require "rust_poc/version"

module RustPoc
  def self.threaded_call(number)
    Ffi.threaded_call(number)
  end
end
