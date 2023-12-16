require "fileutils"

def build_dep(dep)
  dep_dir = lib_dir("c", dep)

  raise "No directory found for dependency: #{dep} at #{dep_dir}" unless Dir.exist?(dep_dir)

  source_files = Dir.glob(File.join(dep_dir, "*.c"))

  # TODO: only build if any source files are newer than the object file
  # TODO this builds the library separately into every day; can it be built into its own build directory and shared?
  system("gcc", "-g", "-Werror", "-c", "-o", "build/#{dep}.o", *source_files)
end

class Runner
  def initialize
  end

  def init
    throw "already initialized" if File.exist?("main.c")
    FileUtils.cp(File.join(template_dir("c"), "main.c"), ".")
  end

  def build(deps)
    Dir.mkdir("build") unless Dir.exist?("build")

    gcc = ["gcc", "-g", "-Werror"]

    aocdeps = deps["aoc"] || []

    if aocdeps.size > 0
      aocdeps.each do |dep|
        raise "Failed to build dependency '#{dep}', quitting" if !build_dep(dep)
      end

      gcc += aocdeps.map { |m| "-I#{lib_dir("c", m)}" }

      gcc << "-Lbuild"
      gcc += aocdeps.map { |m| "-l:#{m}.o" }
    end

    stddeps = deps["std"] || []

    if stddeps.size > 0
      gcc += stddeps.map { |m| "-l#{m}" }
    end

    gcc += ["-o", "build/main", "main.c"]

    system(*gcc)
  end

  def execute(*args)
    system("build/main", *args)
  end

  def run(*args)
    execute(*args)
  end
end
