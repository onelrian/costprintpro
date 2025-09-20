use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Extension,
};
use printpdf::*;
use rust_xlsxwriter::{Workbook, Worksheet, Format};
use uuid::Uuid;

use crate::{
    models::UserInfo,
    services::{job_service::JobService, branding_service::BrandingService},
    utils::errors::AppError,
    AppState,
};

pub async fn export_pdf(
    Path(job_id): Path<Uuid>,
) -> Result<Response, AppError> {
    // Placeholder implementation
    let pdf_data = b"PDF placeholder data".to_vec();

    let headers = [
        (header::CONTENT_TYPE, "application/pdf"),
        (header::CONTENT_DISPOSITION, &format!("attachment; filename=\"quote_{}.pdf\"", job_id)),
    ];

    Ok((StatusCode::OK, headers, pdf_data).into_response())
}

pub async fn export_excel(
    Path(job_id): Path<Uuid>,
) -> Result<Response, AppError> {
    // Placeholder implementation
    let excel_data = b"Excel placeholder data".to_vec();

    let headers = [
        (header::CONTENT_TYPE, "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
        (header::CONTENT_DISPOSITION, &format!("attachment; filename=\"quote_{}.xlsx\"", job_id)),
    ];

    Ok((StatusCode::OK, headers, excel_data).into_response())
}

fn generate_job_pdf(
    job: &crate::models::Job,
    branding: &crate::models::BrandingSettings,
) -> Result<Vec<u8>, AppError> {
    let (doc, page1, layer1) = PdfDocument::new(&format!("Quote - {}", job.title), Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Add company header
    let font = doc.add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| AppError::InternalServerError(format!("PDF font error: {}", e)))?;
    
    current_layer.use_text(&branding.company_name, 24.0, Mm(20.0), Mm(270.0), &font);
    
    // Add job details
    let regular_font = doc.add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| AppError::InternalServerError(format!("PDF font error: {}", e)))?;
    
    let mut y_pos = 250.0;
    
    current_layer.use_text(&format!("Job Title: {}", job.title), 12.0, Mm(20.0), Mm(y_pos), &regular_font);
    y_pos -= 10.0;
    
    current_layer.use_text(&format!("Job Type: {}", job.job_type), 12.0, Mm(20.0), Mm(y_pos), &regular_font);
    y_pos -= 10.0;
    
    current_layer.use_text(&format!("Quantity: {}", job.quantity), 12.0, Mm(20.0), Mm(y_pos), &regular_font);
    y_pos -= 10.0;
    
    current_layer.use_text(&format!("Total Cost: ${:.2}", job.total_cost), 12.0, Mm(20.0), Mm(y_pos), &regular_font);
    y_pos -= 10.0;
    
    current_layer.use_text(&format!("Unit Cost: ${:.2}", job.unit_cost), 12.0, Mm(20.0), Mm(y_pos), &regular_font);
    y_pos -= 20.0;

    // Add cost breakdown
    current_layer.use_text("Cost Breakdown:", 14.0, Mm(20.0), Mm(y_pos), &font);
    y_pos -= 15.0;

    current_layer.use_text(&format!("Paper Cost: ${:.2}", job.cost_breakdown.paper_cost), 10.0, Mm(25.0), Mm(y_pos), &regular_font);
    y_pos -= 8.0;
    
    current_layer.use_text(&format!("Plate Cost: ${:.2}", job.cost_breakdown.plate_cost), 10.0, Mm(25.0), Mm(y_pos), &regular_font);
    y_pos -= 8.0;
    
    current_layer.use_text(&format!("Labor Cost: ${:.2}", job.cost_breakdown.labor_cost), 10.0, Mm(25.0), Mm(y_pos), &regular_font);
    y_pos -= 8.0;
    
    current_layer.use_text(&format!("Binding Cost: ${:.2}", job.cost_breakdown.binding_cost), 10.0, Mm(25.0), Mm(y_pos), &regular_font);
    y_pos -= 8.0;
    
    current_layer.use_text(&format!("Overhead: ${:.2}", job.cost_breakdown.overhead), 10.0, Mm(25.0), Mm(y_pos), &regular_font);

    doc.save_to_bytes()
        .map_err(|e| AppError::InternalServerError(format!("PDF generation error: {}", e)))
}

fn generate_job_excel(
    job: &crate::models::Job,
    branding: &crate::models::BrandingSettings,
) -> Result<Vec<u8>, AppError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Header format
    let header_format = Format::new().set_bold().set_font_size(14);
    let bold_format = Format::new().set_bold();

    // Company header
    worksheet.write_string_with_format(0, 0, &branding.company_name, &header_format)
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;

    // Job details
    let mut row = 2;
    worksheet.write_string_with_format(row, 0, "Job Details", &bold_format)
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;
    
    row += 1;
    worksheet.write_string(row, 0, "Job Title:")
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;
    worksheet.write_string(row, 1, &job.title)
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;
    
    row += 1;
    worksheet.write_string(row, 0, "Job Type:")
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;
    worksheet.write_string(row, 1, &job.job_type.to_string())
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;
    
    row += 1;
    worksheet.write_string(row, 0, "Quantity:")
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;
    worksheet.write_number(row, 1, job.quantity as f64)
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;

    // Cost breakdown
    row += 2;
    worksheet.write_string_with_format(row, 0, "Cost Breakdown", &bold_format)
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;
    
    row += 1;
    worksheet.write_string(row, 0, "Paper Cost:")
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;
    worksheet.write_number(row, 1, job.cost_breakdown.paper_cost.to_string().parse::<f64>().unwrap_or(0.0))
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;
    
    row += 1;
    worksheet.write_string(row, 0, "Plate Cost:")
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;
    worksheet.write_number(row, 1, job.cost_breakdown.plate_cost.to_string().parse::<f64>().unwrap_or(0.0))
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;
    
    row += 1;
    worksheet.write_string(row, 0, "Labor Cost:")
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;
    worksheet.write_number(row, 1, job.cost_breakdown.labor_cost.to_string().parse::<f64>().unwrap_or(0.0))
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;
    
    row += 1;
    worksheet.write_string(row, 0, "Binding Cost:")
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;
    worksheet.write_number(row, 1, job.cost_breakdown.binding_cost.to_string().parse::<f64>().unwrap_or(0.0))
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;
    
    row += 1;
    worksheet.write_string(row, 0, "Overhead:")
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;
    worksheet.write_number(row, 1, job.cost_breakdown.overhead.to_string().parse::<f64>().unwrap_or(0.0))
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;

    // Total
    row += 2;
    worksheet.write_string_with_format(row, 0, "Total Cost:", &bold_format)
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;
    worksheet.write_number_with_format(row, 1, job.total_cost.to_string().parse::<f64>().unwrap_or(0.0), &bold_format)
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;
    
    row += 1;
    worksheet.write_string_with_format(row, 0, "Unit Cost:", &bold_format)
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;
    worksheet.write_number_with_format(row, 1, job.unit_cost.to_string().parse::<f64>().unwrap_or(0.0), &bold_format)
        .map_err(|e| AppError::InternalServerError(format!("Excel write error: {}", e)))?;

    workbook.save_to_buffer()
        .map_err(|e| AppError::InternalServerError(format!("Excel generation error: {}", e)))
}
