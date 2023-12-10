require "fileutils"

def build_dep(dep)
  dep_dir = lib_dir("c", dep)

  raise "No directory found for dependency: #{dep} at #{dep_dir}" unless Dir.exist?(dep_dir)

  source_files = Dir.glob(File.join(dep_dir, "*.c"))

  # TODO: only build if any source files are newer than the object file

  system("gcc", "-g", "-Werror", "-c", "-o", "build/#{dep}.o", *source_files)
end

class Runner
  def init
    FileUtils.cp(File.join(template_dir("c"), "main.c"), ".")
  end

  def build(deps)
    Dir.mkdir("build") unless Dir.exist?("build")

    gcc = ["gcc", "-g", "-Werror"]

    if deps.size > 0
      deps.each do |dep|
        raise "Failed to build dependency '#{dep}', quitting" if !build_dep(dep)
      end

      gcc += deps.map { |m| "-I#{lib_dir("c", m)}" }

      gcc << "-Lbuild"
      gcc += deps.map { |m| "-l:#{m}.o" }
    end

    gcc += ["-o", "build/main", "main.c"]

    system(*gcc)
  end

  def execute
    system("build/main", "../input")
  end

  def run(deps = [])
    build(deps) && execute
  end
end
