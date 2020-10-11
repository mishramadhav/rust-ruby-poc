# frozen_string_literal: true

require 'ffi'

module RustPoc
  module Ffi
    extend FFI::Library
    lib_name = "librust_poc.#{::FFI::Platform::LIBSUFFIX}"
    ffi_lib File.expand_path(lib_name, __dir__)
    attach_function :threaded_call, [:uint], :uint
  end
end
