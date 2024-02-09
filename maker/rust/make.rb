require 'active_support'
require 'active_support/core_ext'
require 'erb'
require 'camelizable'
require 'yaml'

class String
  include Camelizable
  def make_field
    self.gsub(/\./, "_")
  end
  def make_name
    if self == "type"
      "r#type"
    else
      self
    end
  end
end

# only object
def make_body(body, results, array_name=nil)
  body[:properties].each do |it|
    if it[:type] == "object"
      make_body(it, results)
    elsif it[:type] == "array" && it[:items][:type] == "object"
        make_body(it[:items], results, it[:name].singularize)
    elsif it[:type] == "enum_single"
      results << make_expantions(it)
    end
  end
  name = array_name || body[:name] || "body"
  properties = body[:properties]
  erb = ERB.new(File.read("body.erb"))
  results << erb.result(binding)
end

def calc_refs(name, properties, refs)
  properties.each_pair do |key, value|
    if value[:type] == "object"
      if value[:ref].present? && value[:ref] != name.to_s
        refs << value[:ref]
      elsif value[:properties].present?
        calc_refs(key, value[:properties], refs)
      end
    elsif value[:type] == "array" && value[:items][:type] == "object" && value[:items][:ref] != name.to_s
      refs << value[:items][:ref]
    end
  end
end

def make_response(name, properties, independence_flag)
  return ["", [], {}] if properties.blank?
  refs = []
  @enums = {}
  @inner_map = {}
  calc_refs(name, properties, refs)
  refs.uniq!
  class_name = name.make_field.ucc
  erb = ERB.new(File.read("responses.erb"))
  [erb.result(binding), refs, @inner_map]
end

def execute_responses(path)
  m = /responses\/(.+)\.yaml/.match(path)
  name = m[1]
  yml = YAML.load_file(path).deep_symbolize_keys
  properties = yml[:response][:properties]
  independence_flag = true
  res, refs, map = make_response(name, properties, independence_flag)
  map.deep_dup.each_pair do |key, value|
    responses, _, _ = make_response(key.to_s, value.dig(:properties), false)
    res = res + "\n" + responses
  end

  File.write("../../rust/src/responses/#{name}.rs", res)
end

def response_list(yml)
  response_list = []
  responses, refs, map = make_response("response", yml.dig(:response, :properties), false)
  response_list << responses if responses.present?
  if map.present?
    map.each_pair do |key, value|
      responses, _, _ = make_response(key.to_s, value.dig(:properties), false)
      response_list << responses
    end
  end
  [response_list, refs]
end

def execute_apis(path)
  m = /apis\/(.+)\.yaml/.match(path)
  name = m[1]
  yml = YAML.load_file(path).deep_symbolize_keys
  @fields = []
  @enum_flag = false

  queries = yml[:queries] || []
  required_queries = queries.filter{|it| it[:required]}
  non_required_queries =  queries.filter{|it| !it[:required]}

  bodies = []
  make_body(yml[:body], bodies) if yml[:body].present?
  api_struct = ERB.new(File.read("api_struct.erb")).result(binding)
  api_new = ERB.new(File.read("api_new.erb")).result(binding).gsub(/^/, "    ")
  setter = ERB.new(File.read("setter.erb")).result(binding).gsub(/^/, "    ")
  parameters = ERB.new(File.read("parameters.erb")).result(binding).gsub(/^/, "        ")
  responses, responses_refs = response_list(yml)

  erb = ERB.new(File.read("api.erb"))
  File.write("../../rust/src/apis/#{name}.rs", erb.result(binding))
end



Dir.glob('../apis/*.yaml').each do |path|
  execute_apis(path)
end

Dir.glob('../responses/*.yaml').each do |path|
  execute_responses(path)
end