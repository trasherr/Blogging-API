use axum::{Extension, Json, http::StatusCode, extract::{Path, Multipart}};
use chrono::Utc;
use sea_orm::{DatabaseConnection, Set, ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait, Condition, IntoActiveModel};
use tokio::{fs::File, io::AsyncWriteExt};
use uuid::Uuid;

use crate::{utils::api_error::APIError, models::post_models::{CreatePostModel, PostModel}};

use std::io::BufWriter;
use std::num::NonZeroU32;

use image::codecs::png::PngEncoder;
use image::io::Reader as ImageReader;
use image::{ColorType, ImageEncoder};

use fast_image_resize as fr;

pub async fn upload_image_post(
    Extension(db): Extension<DatabaseConnection>,
    Extension(identity): Extension<entity::user::Model>,
    Path(uuid): Path<Uuid>,
    mut multipart: Multipart
) -> Result<(),APIError>{

    while let Some(field) = multipart.next_field().await.unwrap(){

        let field_name = field.name().unwrap().to_string();

        if field_name == "image" {

            let mut post = entity::post::Entity::find().filter(
                Condition::all()
                .add(entity::post::Column::Uuid.eq(uuid))
                .add(entity::post::Column::UserId.eq(identity.id))
            ).one(&db)
            .await.unwrap().unwrap().into_active_model();

            let img_name: i64 = Utc::now().timestamp(); 
            let data = field.bytes().await.unwrap();



            // Read source image from file
            let img = ImageReader::new(std::io::Cursor::new(data))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
            let width = NonZeroU32::new(img.width()).unwrap();
            let height = NonZeroU32::new(img.height()).unwrap();
            let mut src_image = fr::Image::from_vec_u8(
                width,
                height,
                img.to_rgba8().into_raw(),
                fr::PixelType::U8x4,
            ).unwrap();

            // Multiple RGB channels of source image by alpha channel 
            // (not required for the Nearest algorithm)
            let alpha_mul_div = fr::MulDiv::default();
            alpha_mul_div
                .multiply_alpha_inplace(&mut src_image.view_mut())
                .unwrap();

            // Create container for data of destination image
            let dst_width = NonZeroU32::new(480).unwrap();
            let dst_height = NonZeroU32::new(360).unwrap();
            let mut dst_image = fr::Image::new(
                dst_width,
                dst_height,
                src_image.pixel_type(),
            );

            // Get mutable view of destination image data
            let mut dst_view = dst_image.view_mut();

            // Create Resizer instance and resize source image
            // into buffer of destination image
            let mut resizer = fr::Resizer::new(
                fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3),
            );
            resizer.resize(&src_image.view(), &mut dst_view).unwrap();

            // Divide RGB channels of destination image by alpha
            alpha_mul_div.divide_alpha_inplace(&mut dst_view).unwrap();

            // Write destination image as PNG-file
            let mut result_buf = BufWriter::new(Vec::new());
            PngEncoder::new(&mut result_buf)
                .write_image(
                    dst_image.buffer(),
                    dst_width.get(),
                    dst_height.get(),
                    ColorType::Rgba8,
                )
                .unwrap();

            let image_bytes = result_buf.into_inner().unwrap();
            

            let mut file = File::create(format!("./public/uploads/{}.png",img_name)).await.unwrap();
            file.write(&image_bytes).await.unwrap();

            post.image = Set(format!("/uploads/{}.png",img_name));
            post.update(&db).await.unwrap();
            println!("/uploads/{}.png",img_name);
            
        }

        else{
            let data = field.text().await.unwrap();
            println!("field: {}      value: {}",field_name,data);
        }

    }


    Ok(())
}

pub async fn create_post_post(
    Extension(db): Extension<DatabaseConnection>,
    Extension(identity): Extension<entity::user::Model>,
    Json(post_data): Json<CreatePostModel>,

) -> Result<(),APIError> {

    let post_entity = entity::post::ActiveModel {
        title: Set(post_data.title),
        text: Set(post_data.text),
        image: Set(post_data.image),
        created_at: Set(Utc::now().naive_local()),
        user_id: Set(identity.id),
        uuid: Set(Uuid::new_v4()),
        ..Default::default()
    };

    post_entity.insert(&db)
    .await
    .map_err(|_| APIError { message: "Failed to insert".to_owned(), status_code: StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50) })?;

    Ok(())
}

pub async fn get_post_get(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>
) -> Result<Json<PostModel>,APIError> {

    let post: PostModel = entity::post::Entity::find()
    .filter(entity::post::Column::Uuid.eq(uuid))
    .find_also_related(entity::user::Entity)
    .one(&db)
    .await
    .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?
    .ok_or(APIError { message: "Not Found".to_owned(), status_code: StatusCode::NOT_FOUND, error_code: Some(44) })?
    .into();
    
    Ok(Json(post))
}