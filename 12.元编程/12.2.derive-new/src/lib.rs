extern crate proc_macro;
use {
    self::proc_macro::TokenStream,
    proc_macro2,
    quote::*,
    syn::{parse_macro_input, DeriveInput, Token},
};

#[proc_macro_derive(New)]
pub fn derive(input: TokenStream) -> TokenStream {
    // 通过 `parse_macro_input` 宏将 input 解析为 `syn::DeriveInput` 类型的 AST 结构
    let ast = parse_macro_input!(input as DeriveInput);
    // 判断是否为结构体
    let result = match ast.data {
        syn::Data::Struct(ref s) => new_for_struct(&ast, &s.fields),
        _ => panic!("doesn't work with unions yet"),
    };

    // 处理后的最终结果应该属于 `proc_macro2::TokenStream` 类型
    result.into()
    // TokenStream::from(result)
}

// 因为该函数是在 `derive` 函数中被调用, 最终要转换为 `proc_macro2::TokenStream` 类型
fn new_for_struct(ast: &syn::DeriveInput, fields: &syn::Fields) -> proc_macro2::TokenStream {
    match *fields {
        syn::Fields::Named(ref fields) => new_impl(&ast, Some(&fields.named), true),
        syn::Fields::Unit => new_impl(&ast, None, false),
        syn::Fields::Unnamed(ref fields) => new_impl(&ast, Some(&fields.unnamed), false),
    }
}

fn new_impl(
    ast: &syn::DeriveInput,
    // 结构体字段: `syn::Field`
    // 逗号标记: `Token![,]`
    fields: Option<&syn::punctuated::Punctuated<syn::Field, Token![,]>>,
    named: bool,
) -> proc_macro2::TokenStream {
    let struct_name = &ast.ident;

    // 12-67
    // 判断是否单元结构体
    let unit = fields.is_none();
    // 自动推断单元结构体的字段值为单元值
    let empty = Default::default();

    // 将 `fields` 转换为 `FieldExt` 的数组集合
    let fields: Vec<_> = fields
        .unwrap_or(&empty)
        .iter()
        .enumerate()
        .map(|(i, f)| FieldExt::new(f, i, named))
        .collect();

    let args = fields.iter().map(|f| f.as_arg());
    let inits = fields.iter().map(|f| f.as_init());

    let inits = if unit {
        quote!()
    } else if named {
        quote![ { #(#inits),* } ]
    } else {
        quote![ ( #(#inits),* ) ]
    };


    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let (new, doc) = (
        // 将字符串转换为 syn 的 AST 以备使用
        // 在 quote! 进行代码生成的时候不允许使用直接字符串, 文档 doc 除外
        syn::Ident::new("new", proc_macro2::Span::call_site()),
        format!("Constructs a new `{}`.", struct_name),
    );

    // new 函数中传入的参数
    // let args = "TODO";
    // 字段信息
    // let inits = "TODO";

    // 相当于生成代码的模板
    // *: 重复 0 次或多次
    quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #[doc = #doc]
            pub fn #new(#(#args),*) -> Self {
                #struct_name #inits
            }
        }
    }
}


// 12-65
struct FieldExt<'a> {
    // 存储结构体字段的类型信息
    ty: &'a syn::Type,
    // 存储结构体字段的字段名
    ident: syn::Ident,
    // 是否为命名结构体
    named: bool,
}


impl<'a> FieldExt<'a> {
    // 参数: 1. 记录了结构体字段的信息, 2: 记录元组结构体字段位置
    pub fn new(field: &'a syn::Field, idx: usize, named: bool) -> FieldExt<'a> {
        FieldExt {
            ty: &field.ty,
            ident: if named {
                field.ident.clone().unwrap()
            } else {
                syn::Ident::new(&format!("f{}", idx), proc_macro2::Span::call_site())
            },
            named,
        }
    }

    // 得到参数信息的结构
    pub fn as_arg(&self) -> proc_macro2::TokenStream {
        // 参数名称
        let f_name = &self.ident;
        // 参数类型
        let ty = &self.ty;
        quote!(#f_name: #ty)
    }

    // 得到字段信息的结构
    pub fn as_init(&self) -> proc_macro2::TokenStream {
        let f_name = &self.ident;
        let init = quote!(#f_name);

        if self.named {
            quote!(#f_name: #init)
        } else {
            quote!(#init)
        }
    }
}
