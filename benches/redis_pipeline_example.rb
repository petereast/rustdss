require 'rubygems'
require 'redis'

def bench(descr)
    start = Time.now
    yield
    puts "#{descr} #{Time.now-start} seconds"
end

def without_pipelining
    r = Redis.new
    10000.times {
        r.incr "b"
    }
    10000.times {
      r.decr "b"
    }

    puts "This should be zero: ", (r.get "b")
end

def with_pipelining
    r = Redis.new
    r.pipelined {
        10000.times {
          r.incr "a"
        }
        10000.times {
          r.decr "a"
        }

    }
    puts "This should be zero: ", (r.get "a")
end

bench("without pipelining") {
    without_pipelining
 }
#
bench("with pipelining") {
    with_pipelining
}

