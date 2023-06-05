use entity::{hotkey, settings};
use sea_orm_migration::prelude::*;

use sea_orm_migration::sea_orm::entity::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        settings::ActiveModel {
            dark_mode: Set(true),
            notification: Set(true),
            startup: Set(true),
            synchronize: Set(true),
            synchronize_time: Set(600),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("windowDisplayToggle".to_string()),
            ctrl: Set(true),
            alt: Set(false),
            shift: Set(false),
            key: Set("Y".to_string()),
            status: Set(true),
            name: Set("Clippy Display Toggle".to_string()),
            icon: Set("\"<svg stroke-width=\\\"0\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" viewBox=\\\"0 0 576 512\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path d=\\\"M64 112c-8.8 0-16 7.2-16 16v256c0 8.8 7.2 16 16 16h448c8.8 0 16-7.2 16-16V128c0-8.8-7.2-16-16-16H64zM0 128c0-35.3 28.7-64 64-64h448c35.3 0 64 28.7 64 64v256c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V128zm176 192h224c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16H176c-8.8 0-16-7.2-16-16v-16c0-8.8 7.2-16 16-16zm-72-72c0-8.8 7.2-16 16-16h16c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16h-16c-8.8 0-16-7.2-16-16v-16zm16-96h16c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16h-16c-8.8 0-16-7.2-16-16v-16c0-8.8 7.2-16 16-16zm64 96c0-8.8 7.2-16 16-16h16c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16h-16c-8.8 0-16-7.2-16-16v-16zm16-96h16c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16h-16c-8.8 0-16-7.2-16-16v-16c0-8.8 7.2-16 16-16zm64 96c0-8.8 7.2-16 16-16h16c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16h-16c-8.8 0-16-7.2-16-16v-16zm16-96h16c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16h-16c-8.8 0-16-7.2-16-16v-16c0-8.8 7.2-16 16-16zm64 96c0-8.8 7.2-16 16-16h16c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16h-16c-8.8 0-16-7.2-16-16v-16zm16-96h16c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16h-16c-8.8 0-16-7.2-16-16v-16c0-8.8 7.2-16 16-16zm64 96c0-8.8 7.2-16 16-16h16c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16h-16c-8.8 0-16-7.2-16-16v-16zm16-96h16c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16h-16c-8.8 0-16-7.2-16-16v-16c0-8.8 7.2-16 16-16z\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("recentClipboards".to_string()),
            ctrl: Set(false),
            alt: Set(false),
            shift: Set(false),
            key: Set("R".to_string()),
            status: Set(true),
            name: Set("Recent Clipboards".to_string()),
            icon: Set("\"<svg stroke-width=\\\"0\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"currentColor\\\" viewBox=\\\"0 0 16 16\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path fill-rule=\\\"evenodd\\\" d=\\\"M13.507 12.324a7 7 0 0 0 .065-8.56A7 7 0 0 0 2 4.393V2H1v3.5l.5.5H5V5H2.811a6.008 6.008 0 1 1-.135 5.77l-.887.462a7 7 0 0 0 11.718 1.092zm-3.361-.97.708-.707L8 7.792V4H7v4l.146.354 3 3z\\\" clip-rule=\\\"evenodd\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("starredClipboards".to_string()),
            ctrl: Set(false),
            alt: Set(false),
            shift: Set(false),
            key: Set("S".to_string()),
            status: Set(true),
            name: Set("Starred Clipboards".to_string()),
            icon: Set("<svg stroke-width=\"0\" height=\"1em\" width=\"1em\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 1024 1024\" color=\"currentColor\" style=\"overflow: visible;\"><path d=\"m908.1 353.1-253.9-36.9L540.7 86.1c-3.1-6.3-8.2-11.4-14.5-14.5-15.8-7.8-35-1.3-42.9 14.5L369.8 316.2l-253.9 36.9c-7 1-13.4 4.3-18.3 9.3a32.05 32.05 0 0 0 .6 45.3l183.7 179.1-43.4 252.9a31.95 31.95 0 0 0 46.4 33.7L512 754l227.1 119.4c6.2 3.3 13.4 4.4 20.3 3.2 17.4-3 29.1-19.5 26.1-36.9l-43.4-252.9 183.7-179.1c5-4.9 8.3-11.3 9.3-18.3 2.7-17.5-9.5-33.7-27-36.3z\"></path></svg>".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("history".to_string()),
            ctrl: Set(false),
            alt: Set(false),
            shift: Set(false),
            key: Set("H".to_string()),
            status: Set(true),
            name: Set("History".to_string()),
            icon: Set("\"<svg stroke-width=\\\"0\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" viewBox=\\\"0 0 1024 1024\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path d=\\\"M909.6 854.5 649.9 594.8C690.2 542.7 712 479 712 412c0-80.2-31.3-155.4-87.9-212.1-56.6-56.7-132-87.9-212.1-87.9s-155.5 31.3-212.1 87.9C143.2 256.5 112 331.8 112 412c0 80.1 31.3 155.5 87.9 212.1C256.5 680.8 331.8 712 412 712c67 0 130.6-21.8 182.7-62l259.7 259.6a8.2 8.2 0 0 0 11.6 0l43.6-43.5a8.2 8.2 0 0 0 0-11.6zM570.4 570.4C528 612.7 471.8 636 412 636s-116-23.3-158.4-65.6C211.3 528 188 471.8 188 412s23.3-116.1 65.6-158.4C296 211.3 352.2 188 412 188s116.1 23.2 158.4 65.6S636 352.2 636 412s-23.3 116.1-65.6 158.4z\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("viewMore".to_string()),
            ctrl: Set(false),
            alt: Set(false),
            shift: Set(false),
            key: Set("M".to_string()),
            status: Set(true),
            name: Set("View more".to_string()),
            icon: Set("\"<svg stroke-width=\\\"0\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" viewBox=\\\"0 0 448 512\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path d=\\\"M8 256a56 56 0 1 1 112 0 56 56 0 1 1-112 0zm160 0a56 56 0 1 1 112 0 56 56 0 1 1-112 0zm216-56a56 56 0 1 1 0 112 56 56 0 1 1 0-112z\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("syncClipboardHistory".to_string()),
            ctrl: Set(false),
            alt: Set(false),
            shift: Set(false),
            key: Set("P".to_string()),
            status: Set(true),
            name: Set("Sync Clipboard History".to_string()),
            icon: Set("\"<svg stroke-width=\\\"2\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"none\\\" stroke=\\\"currentColor\\\" stroke-linecap=\\\"round\\\" stroke-linejoin=\\\"round\\\" viewBox=\\\"0 0 24 24\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path d=\\\"M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z\\\"></path><path d=\\\"M17 21v-8H7v8M7 3v5h8\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("preferences".to_string()),
            ctrl: Set(false),
            alt: Set(false),
            shift: Set(false),
            key: Set("O".to_string()),
            status: Set(true),
            name: Set("Preferences".to_string()),
            icon: Set("\"<svg stroke-width=\\\"0\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" viewBox=\\\"0 0 24 24\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path d=\\\"M12 16c2.206 0 4-1.794 4-4s-1.794-4-4-4-4 1.794-4 4 1.794 4 4 4zm0-6c1.084 0 2 .916 2 2s-.916 2-2 2-2-.916-2-2 .916-2 2-2z\\\"></path><path d=\\\"m2.845 16.136 1 1.73c.531.917 1.809 1.261 2.73.73l.529-.306A8.1 8.1 0 0 0 9 19.402V20c0 1.103.897 2 2 2h2c1.103 0 2-.897 2-2v-.598a8.132 8.132 0 0 0 1.896-1.111l.529.306c.923.53 2.198.188 2.731-.731l.999-1.729a2.001 2.001 0 0 0-.731-2.732l-.505-.292a7.718 7.718 0 0 0 0-2.224l.505-.292a2.002 2.002 0 0 0 .731-2.732l-.999-1.729c-.531-.92-1.808-1.265-2.731-.732l-.529.306A8.1 8.1 0 0 0 15 4.598V4c0-1.103-.897-2-2-2h-2c-1.103 0-2 .897-2 2v.598a8.132 8.132 0 0 0-1.896 1.111l-.529-.306c-.924-.531-2.2-.187-2.731.732l-.999 1.729a2.001 2.001 0 0 0 .731 2.732l.505.292a7.683 7.683 0 0 0 0 2.223l-.505.292a2.003 2.003 0 0 0-.731 2.733zm3.326-2.758A5.703 5.703 0 0 1 6 12c0-.462.058-.926.17-1.378a.999.999 0 0 0-.47-1.108l-1.123-.65.998-1.729 1.145.662a.997.997 0 0 0 1.188-.142 6.071 6.071 0 0 1 2.384-1.399A1 1 0 0 0 11 5.3V4h2v1.3a1 1 0 0 0 .708.956 6.083 6.083 0 0 1 2.384 1.399.999.999 0 0 0 1.188.142l1.144-.661 1 1.729-1.124.649a1 1 0 0 0-.47 1.108c.112.452.17.916.17 1.378 0 .461-.058.925-.171 1.378a1 1 0 0 0 .471 1.108l1.123.649-.998 1.729-1.145-.661a.996.996 0 0 0-1.188.142 6.071 6.071 0 0 1-2.384 1.399A1 1 0 0 0 13 18.7l.002 1.3H11v-1.3a1 1 0 0 0-.708-.956 6.083 6.083 0 0 1-2.384-1.399.992.992 0 0 0-1.188-.141l-1.144.662-1-1.729 1.124-.651a1 1 0 0 0 .471-1.108z\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("about".to_string()),
            ctrl: Set(false),
            alt: Set(false),
            shift: Set(false),
            key: Set("O".to_string()),
            status: Set(true),
            name: Set("About".to_string()),
            icon: Set("\"<svg stroke-width=\\\"0\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" viewBox=\\\"0 0 1024 1024\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path d=\\\"M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm0 820c-205.4 0-372-166.6-372-372s166.6-372 372-372 372 166.6 372 372-166.6 372-372 372z\\\"></path><path d=\\\"M464 336a48 48 0 1 0 96 0 48 48 0 1 0-96 0zm72 112h-48c-4.4 0-8 3.6-8 8v272c0 4.4 3.6 8 8 8h48c4.4 0 8-3.6 8-8V456c0-4.4-3.6-8-8-8z\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("exit".to_string()),
            ctrl: Set(false),
            alt: Set(false),
            shift: Set(false),
            key: Set("X".to_string()),
            status: Set(true),
            name: Set("Exit".to_string()),
            icon: Set("\"<svg stroke-width=\\\"2\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"none\\\" stroke=\\\"currentColor\\\" stroke-linecap=\\\"round\\\" stroke-linejoin=\\\"round\\\" viewBox=\\\"0 0 24 24\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path d=\\\"M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4M16 17l5-5-5-5M21 12H9\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("toggleDevTool".to_string()),
            ctrl: Set(false),
            alt: Set(false),
            shift: Set(false),
            key: Set("F11".to_string()),
            status: Set(true),
            name: Set("Toggle Dev Tools".to_string()),
            icon: Set("\"<svg stroke-width=\\\"0\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"currentColor\\\" viewBox=\\\"0 0 16 16\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path fill-rule=\\\"evenodd\\\" d=\\\"m14.773 3.485-.78-.184-2.108 2.096-1.194-1.216 2.056-2.157-.18-.792a4.42 4.42 0 0 0-1.347-.228 3.64 3.64 0 0 0-1.457.28 3.824 3.824 0 0 0-1.186.84 3.736 3.736 0 0 0-.875 1.265 3.938 3.938 0 0 0 0 2.966 335.341 335.341 0 0 0-6.173 6.234c-.21.275-.31.618-.284.963a1.403 1.403 0 0 0 .464.967c.124.135.272.247.437.328.17.075.353.118.538.127.316-.006.619-.126.854-.337 1.548-1.457 4.514-4.45 6.199-6.204.457.194.948.294 1.444.293a3.736 3.736 0 0 0 2.677-1.133 3.885 3.885 0 0 0 1.111-2.73 4.211 4.211 0 0 0-.196-1.378zM2.933 13.928a.31.31 0 0 1-.135.07.437.437 0 0 1-.149 0 .346.346 0 0 1-.144-.057.336.336 0 0 1-.114-.11c-.14-.143-.271-.415-.14-.568 1.37-1.457 4.191-4.305 5.955-6.046.1.132.21.258.328.376.118.123.245.237.38.341-1.706 1.75-4.488 4.564-5.98 5.994zm11.118-9.065c.002.765-.296 1.5-.832 2.048a2.861 2.861 0 0 1-4.007 0 2.992 2.992 0 0 1-.635-3.137A2.748 2.748 0 0 1 10.14 2.18a2.76 2.76 0 0 1 1.072-.214h.254L9.649 3.839v.696l1.895 1.886h.66l1.847-1.816v.258zM3.24 6.688h1.531l.705.717.678-.674-.665-.678V6.01l.057-1.649-.22-.437-2.86-1.882-.591.066-.831.849-.066.599 1.838 2.918.424.215zm-.945-3.632L4.609 4.58 4.57 5.703H3.494L2.002 3.341l.293-.285zm7.105 6.96.674-.673 3.106 3.185a1.479 1.479 0 0 1 0 2.039 1.404 1.404 0 0 1-1.549.315 1.31 1.31 0 0 1-.437-.315l-3.142-3.203.679-.678 3.132 3.194a.402.402 0 0 0 .153.105.477.477 0 0 0 .359 0 .403.403 0 0 0 .153-.105.436.436 0 0 0 .1-.153.525.525 0 0 0 .036-.184.547.547 0 0 0-.035-.184.436.436 0 0 0-.1-.153L9.4 10.016z\\\" clip-rule=\\\"evenodd\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("scrollToTop".to_string()),
            ctrl: Set(false),
            alt: Set(false),
            shift: Set(false),
            key: Set("E".to_string()),
            status: Set(true),
            name: Set("Scroll to Top".to_string()),
            icon: Set("\"<svg stroke-width=\\\"0\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" viewBox=\\\"0 0 384 512\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path d=\\\"M214.6 41.4c-12.5-12.5-32.8-12.5-45.3 0l-160 160c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0L160 141.2V448c0 17.7 14.3 32 32 32s32-14.3 32-32V141.2l105.4 105.4c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3l-160-160z\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}