use crate::render::IBackend;

use ash::vk::{self, API_VERSION_1_2};
use ash::extensions::khr::Win32Surface;
use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;


use std::ffi::CString;
use std::ptr;

const WINDOW_TITLE: &'static str = "edu";
const APPLICATION_VERSION: u32 = 1;
const ENGINE_VERSION: u32 = 1;


pub struct Vulkan {
	_entry: ash::Entry,
	instance: ash::Instance,
}

impl Vulkan {
	pub fn new() -> Vulkan 
	{
		unsafe {
		let entry = ash::Entry::new().unwrap();
		let instance = Vulkan::create_instance(&entry);
		Vulkan{_entry : entry, instance}
		}
	}

	pub fn required_extension_names() -> Vec<*const i8> {
		vec![
		    Surface::name().as_ptr(),
		    Win32Surface::name().as_ptr(),
		    DebugUtils::name().as_ptr(),
		]
	    }

	fn create_instance(entry: &ash::Entry) -> ash::Instance {
        
		let app_name = CString::new(WINDOW_TITLE).unwrap();
		let engine_name = CString::new("Vulkan Engine").unwrap();
		let app_info = vk::ApplicationInfo {
		    s_type: vk::StructureType::APPLICATION_INFO,
		    p_next: ptr::null(),
		    p_application_name: app_name.as_ptr(),
		    application_version: APPLICATION_VERSION,
		    p_engine_name: engine_name.as_ptr(),
		    engine_version: ENGINE_VERSION,
		    api_version: API_VERSION_1_2,
		};
	
		let extension_names = Vulkan::required_extension_names();
	
		let create_info = vk::InstanceCreateInfo {
		    s_type: vk::StructureType::INSTANCE_CREATE_INFO,
		    p_next: ptr::null(),
		    flags: vk::InstanceCreateFlags::empty(),
		    p_application_info: &app_info,
		    pp_enabled_layer_names: ptr::null(),
		    enabled_layer_count: 0,
		    pp_enabled_extension_names: extension_names.as_ptr(),
		    enabled_extension_count: extension_names.len() as u32,
		};
	
		let instance: ash::Instance = unsafe {
		    entry
			.create_instance(&create_info, None)
			.expect("Failed to create instance!")
		};
	
		instance
	    }
}
impl IBackend for Vulkan {
	fn init() {}
}