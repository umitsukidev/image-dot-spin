use nannou::prelude::*;
use std::path::PathBuf;

struct Model {
    texture: Option<Handle<Image>>,
    texture_path: Option<PathBuf>,
    texture_error: Option<String>,
    file_dialog_task: Option<Task<Option<PathBuf>>>,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .primary()
        .size(1200, 900)
        .resizable(false)
        .view(view)
        .build();

    Model {
        texture: None,
        texture_path: None,
        texture_error: None,
        file_dialog_task: None,
    }
}

fn update(app: &App, model: &mut Model) {
    let selected_path = model
        .file_dialog_task
        .as_mut()
        .and_then(|task| block_on(future::poll_once(task)));

    if let Some(selected_path) = selected_path {
        model.file_dialog_task = None;

        if let Some(path) = selected_path {
            match load_texture(app, &path) {
                Ok(texture) => {
                    model.texture = Some(texture);
                    model.texture_path = Some(path);
                    model.texture_error = None;
                }
                Err(error) => model.texture_error = Some(error),
            }
        }
    }

    let egui = app.egui();

    egui.set_global_style(egui::Style {
        visuals: egui::Visuals {
            window_shadow: egui::Shadow::NONE,
            ..Default::default()
        },
        interaction: egui::style::Interaction {
            selectable_labels: false,
            ..Default::default()
        },
        ..Default::default()
    });

    egui::Window::new("Texture").show(&egui, |ui| {
        if ui
            .add_enabled(
                model.file_dialog_task.is_none(),
                egui::Button::new("Choose image..."),
            )
            .clicked()
        {
            model.file_dialog_task = Some(IoTaskPool::get().spawn(async {
                rfd::AsyncFileDialog::new()
                    .add_filter("Images", &["png", "jpg", "jpeg"])
                    .pick_file()
                    .await
                    .map(|file| file.path().to_owned())
            }));
        }

        if model.file_dialog_task.is_some() {
            ui.label("Opening file dialog...");
        }

        if let Some(path) = &model.texture_path {
            ui.label(format!("Selected: {}", path.display()));
        }

        if let Some(error) = &model.texture_error {
            ui.colored_label(egui::Color32::RED, error);
        }
    });
}

fn load_texture(app: &App, path: &std::path::Path) -> Result<Handle<Image>, String> {
    let bytes = std::fs::read(path).map_err(|error| error.to_string())?;
    let extension = path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .ok_or_else(|| "The selected file has no supported image extension".to_string())?;
    let image = Image::from_buffer(
        &bytes,
        bevy_image::ImageType::Extension(extension),
        bevy_image::CompressedImageFormats::NONE,
        true,
        ImageSampler::Default,
        bevy_asset::RenderAssetUsages::default(),
    )
    .map_err(|error| error.to_string())?;

    Ok(app.asset_server().add(image))
}

fn view(app: &App, _model: &Model) {
    let draw = app.draw();

    draw.background().color(WHITE);
}
