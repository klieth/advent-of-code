require "debug"

def parse(input)
  input.lines.map do |line|
    Node.new(line.strip.split(',').map { Integer(it) })
  end
end

class Node
  attr_reader :loc
  attr_accessor :circuit

  def initialize(loc)
    @loc = loc
    @cirucit = nil
  end

  def -(other)
    Math.sqrt(loc.zip(other.loc).map do |a, b|
      (a - b).abs2
    end.sum)
  end

  def to_s
    "Node#{loc}"
  end

  def inspect
    "Node:#{object_id}#{loc}"
  end
end

class Edge
  attr_reader :ends, :distance

  def initialize(*ends)
    @ends = ends

    @distance = @ends[0] - @ends[1]
  end
end

class CircuitSet < Set
  def initialize
    super
    compare_by_identity
  end

  def update(node_a, node_b)
    if node_a.circuit == nil && node_b.circuit == nil
      c = [node_a, node_b]

      node_a.circuit = c
      node_b.circuit = c

      self << c
    elsif node_a.circuit == nil
      c = node_b.circuit
      c << node_a
      node_a.circuit = c
    elsif node_b.circuit == nil
      c = node_a.circuit
      c << node_b
      node_b.circuit = c
    else
      return if node_a.circuit.equal?(node_b.circuit)
      c = node_b.circuit
      self.delete(c)
      c.each do |node|
        node.circuit = node_a.circuit
      end
      node_a.circuit.append(*c)
    end
  end
end

def part1(data, edges)
  circuits = CircuitSet.new

  edges[..1000].each do |edge|
    node_a, node_b = edge.ends

    circuits.update(node_a, node_b)
  end

  circuits.map(&:length).sort.last(3).reduce(:*)
end

def part2(data, edges)
  circuits = CircuitSet.new

  res = nil

  edges.each do |edge|
    node_a, node_b = edge.ends

    circuits.update(node_a, node_b)

    if circuits.length == 1 && circuits.any? { it.length == data.length }
      res = node_a.loc[0] * node_b.loc[0]
      break
    end
  end

  res
end

def report_timed(label = nil, print_result = true, &block)
  mark = Time.now
  res = yield
  elapsed = Time.now - mark

  out = "#{label}: " || ""
  out += res.to_s if print_result

  puts "#{out}: in #{elapsed} seconds"

  res
end

test1 = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"

hardcoded_data = {
  "test1" => test1,
}

input_name = ARGV[0]

data = report_timed("read", false) do
  if hardcoded_data.has_key?(input_name)
    hardcoded_data[input_name]
  else
    folder = File.dirname(__FILE__)
    path = File.expand_path(File.join(folder, "../#{input_name}"))
    File.read(path)
  end
end

data = report_timed("parse", false) { parse(data) }

edges = []

report_timed("calculate sorted edges", false) do
  data.each.with_index do |a, i|
    data[i+1..].each do |b|
      edges << Edge.new(a, b)
    end
  end

  edges.sort_by!(&:distance)
end

report_timed("part 1") { part1(data, edges) }
data.each { it.circuit = nil }
report_timed("part 2") { part2(data, edges) }
