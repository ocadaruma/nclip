use anyhow::anyhow;
use anyhow::Result;
use cocoa::appkit::NSPasteboardTypePNG;
use cocoa::base::nil;
use cocoa::foundation::NSInteger;
use objc::runtime::{Object, BOOL, YES};
use objc::{class, msg_send, sel, sel_impl};
use objc_foundation::{INSArray, INSData, NSArray, NSData, NSObject};
use objc_id::Id;

pub fn read_image() -> Result<Vec<u8>> {
    let pasteboard: *const Object = unsafe { msg_send![class!(NSPasteboard), generalPasteboard] };
    if pasteboard.is_null() {
        return Err(anyhow!("Failed to get pasteboard"));
    }
    let data: Id<NSData> = unsafe {
        let obj: *mut NSData = msg_send![pasteboard, dataForType: NSPasteboardTypePNG];
        if obj.is_null() {
            return Err(anyhow!("Failed to get pasteboard data as PNG"));
        }
        Id::from_ptr(obj)
    };

    Ok(data.bytes().to_vec())
}

pub fn write_image(data: Vec<u8>) -> Result<()> {
    let data = NSData::from_vec(data);
    let pasteboard: *const Object = unsafe { msg_send![class!(NSPasteboard), generalPasteboard] };
    if pasteboard.is_null() {
        return Err(anyhow!("Failed to get pasteboard"));
    }

    unsafe {
        let types: Id<NSArray<NSObject>> =
            NSArray::from_vec(vec![Id::from_ptr(NSPasteboardTypePNG as *mut NSObject)]);
        let _: NSInteger = msg_send![pasteboard, declareTypes:types owner:nil];
    }
    let ret: BOOL = unsafe { msg_send![pasteboard, setData:data forType:NSPasteboardTypePNG] };
    if ret == YES {
        Ok(())
    } else {
        Err(anyhow!("Failed to set pasteboard data as PNG"))
    }
}
