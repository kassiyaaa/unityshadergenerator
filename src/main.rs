pub struct ShaderGenerator {
    pub name: String,
    pub properties: Vec<ShaderProperty>,
    pub vertex_code: String,
    pub fragment_code: String,
}

pub struct ShaderProperty {
    pub name: String,
    pub property_type: String,
    pub default_value: String,
}

impl ShaderGenerator {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            properties: Vec::new(),
            vertex_code: String::new(),
            fragment_code: String::new(),
        }
    }

    pub fn add_property(&mut self, name: &str, prop_type: &str, default: &str) {
        self.properties.push(ShaderProperty {
            name: name.to_string(),
            property_type: prop_type.to_string(),
            default_value: default.to_string(),
        });
    }

    pub fn generate_unity_shader(&self) -> String {
        let mut shader = format!("Shader \"{}\"\n{{\n", self.name);
        
        // Properties
        shader.push_str("    Properties\n    {\n");
        for prop in &self.properties {
            shader.push_str(&format!("        {} (\"{}\", {}) = {}\n", 
                prop.name, prop.name, prop.property_type, prop.default_value));
        }
        shader.push_str("    }\n\n");
        
        // SubShader
        shader.push_str("    SubShader\n    {\n");
        shader.push_str("        Tags { \"RenderType\"=\"Opaque\" }\n");
        shader.push_str("        LOD 100\n\n");
        
        shader.push_str("        Pass\n        {\n");
        shader.push_str("            PROGRAM\n");
        shader.push_str("            #pragma vertex vert\n");
        shader.push_str("            #pragma fragment frag\n\n");
        
        // Vertex shader
        shader.push_str(&self.vertex_code);
        shader.push_str("\n\n");
        
        // Fragment shader
        shader.push_str(&self.fragment_code);
        shader.push_str("\n\n");
        
        shader.push_str("            ENDCG\n");
        shader.push_str("        }\n");
        shader.push_str("    }\n");
        shader.push_str("}\n");
        
        shader
    }
}

fn main() {
    let mut generator = ShaderGenerator::new("Custom/MyShader");
    
    generator.add_property("_MainTex", "2D", "\"white\" {}");
    generator.add_property("_Color", "Color", "(1,1,1,1)");
    
    generator.vertex_code = r#"
            #include "UnityCG.cginc"
            
            struct appdata
            {
                float4 vertex : POSITION;
                float2 uv : TEXCOORD0;
            };

            struct v2f
            {
                float2 uv : TEXCOORD0;
                float4 vertex : SV_POSITION;
            };

            sampler2D _MainTex;
            float4 _MainTex_ST;
            
            v2f vert (appdata v)
            {
                v2f o;
                o.vertex = UnityObjectToClipPos(v.vertex);
                o.uv = TRANSFORM_TEX(v.uv, _MainTex);
                return o;
            }"#.to_string();
    
    generator.fragment_code = r#"
            fixed4 _Color;
            
            fixed4 frag (v2f i) : SV_Target
            {
                fixed4 col = tex2D(_MainTex, i.uv) * _Color;
                return col;
            }"#.to_string();
    
    println!("{}", generator.generate_unity_shader());
}