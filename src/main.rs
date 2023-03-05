use std::io::Read;

use dokan::{FileSystemMounter, MountFlags, MountOptions};
use widestring::{U16CString, U16CStr};
use winapi::um::winnt;

const STATUS_INVALID_DEVICE_REQUEST:i32 = 0x00000005;
const STATUS_NOT_IMPLEMENTED:i32 = STATUS_INVALID_DEVICE_REQUEST;

fn main() {

	let mount_point = U16CString::from_str("Y")?;


	let mut flags = MountFlags::ALT_STREAM;
	// if matches.is_present("dokan_debug") {
		flags |= MountFlags::DEBUG | MountFlags::STDERR;
	// }
	// if matches.is_present("removable") {
	// 	flags |= MountFlags::REMOVABLE;
	// }

	let options = MountOptions {
		// single_thread: matches.is_present("single_thread"),
		flags,
		..Default::default()
	};
	let handler = WebDavPtFsHandler::new();


    dokan::init();
	let mut mounter = FileSystemMounter::new(&handler, &mount_point, &options);

	println!("File system will mount...");

	let file_system = mounter.mount()?;

	let file_system = mounter.mount()?;

	// Another thread can unmount the file system.
	let mount_point = mount_point.clone();
	ctrlc::set_handler(move || {
		if dokan::unmount(&mount_point) {
			println!("File system will unmount...")
		} else {
			eprintln!("Failed to unmount file system.");
		}
	})
	.expect("failed to set Ctrl-C handler");

	println!("File system is mounted, press Ctrl-C to unmount.");

	drop(file_system);

	println!("File system is unmounted.");
    dokan::shutdown();
    // dokan::list_mount_points(unc_only)
}

struct WebDavPtFsHandler{
	webdav_server_url: String,
	webdav_port: i32,
	user_name: Option<String>,
	password: Option<String>,
}

impl WebDavPtFsHandler {
	fn new() -> Self {
		Self { webdav_server_url: (), webdav_port: (), user_name: None, password: None }
	}
}


impl <'c, 'h: 'c>  dokan::FileSystemHandler<'c, 'h> for WebDavPtFsHandler {
    type Context = ();

    fn create_file(
		&'h self,
		file_name: &U16CStr,
		security_context: &dokan::IO_SECURITY_CONTEXT,
		desired_access: winnt::ACCESS_MASK,
		file_attributes: u32,
		share_access: u32,
		create_disposition: u32,
		create_options: u32,
		info: &mut dokan::OperationInfo<'c, 'h, Self>,
	) -> dokan::OperationResult<dokan::CreateFileInfo<Self::Context>> {
		Err(-1)
	}

    fn cleanup(
		&'h self,
		file_name: &U16CStr,
		info: &dokan::OperationInfo<'c, 'h, Self>,
		context: &'c Self::Context,
	) {
	}

    fn close_file(
		&'h self,
		file_name: &U16CStr,
		info: &dokan::OperationInfo<'c, 'h, Self>,
		context: &'c Self::Context,
	) {
	}

    fn read_file(
		&'h self,
		file_name: &U16CStr,
		offset: i64,
		buffer: &mut [u8],
		info: &dokan::OperationInfo<'c, 'h, Self>,
		context: &'c Self::Context,
	) -> dokan::OperationResult<u32> {
		let webdav_client = rustydav::client::Client::init("username", "pwd");
		// async || {
		// 	let res= webdav_client.get("").await;
		// }
		let res = webdav_client.get("");
		if let Ok(res) = res {
			// let read_result = res.bytes();
			// if let Ok(content) = read_result {
			// 	content.
			// } 
			return res.read(buffer).map_or_else(|_|Err(-1i32), |num| Ok(num as u32));


		}
		Err(-1)
		

	}

    fn write_file(
		&'h self,
		file_name: &U16CStr,
		offset: i64,
		buffer: &[u8],
		info: &dokan::OperationInfo<'c, 'h, Self>,
		context: &'c Self::Context,
	) -> dokan::OperationResult<u32> {
		
		let webdav_client = rustydav::client::Client::init("username", "pwd");
		// async || {
		// 	let res= webdav_client.get("").await;
		// }
		webdav_client.put(buffer, "").map_or_else(|_| Err(-1), |_| Ok(0))

	}

    fn flush_file_buffers(
		&'h self,
		file_name: &U16CStr,
		info: &dokan::OperationInfo<'c, 'h, Self>,
		context: &'c Self::Context,
	) -> dokan::OperationResult<()> {
		Ok(())
	}

    fn get_file_information(
		&'h self,
		file_name: &U16CStr,
		info: &dokan::OperationInfo<'c, 'h, Self>,
		context: &'c Self::Context,
	) -> dokan::OperationResult<dokan::FileInfo> {
		Err(STATUS_NOT_IMPLEMENTED)
	}

    fn find_files(
		&'h self,
		file_name: &U16CStr,
		fill_find_data: impl FnMut(&dokan::FindData) -> dokan::FillDataResult,
		info: &dokan::OperationInfo<'c, 'h, Self>,
		context: &'c Self::Context,
	) -> dokan::OperationResult<()> {
		Err(STATUS_NOT_IMPLEMENTED)
	}

    fn find_files_with_pattern(
		&'h self,
		file_name: &U16CStr,
		pattern: &U16CStr,
		fill_find_data: impl FnMut(&dokan::FindData) -> dokan::FillDataResult,
		info: &dokan::OperationInfo<'c, 'h, Self>,
		context: &'c Self::Context,
	) -> dokan::OperationResult<()> {
		Err(STATUS_NOT_IMPLEMENTED)
	}

    fn set_file_attributes(
		&'h self,
		file_name: &U16CStr,
		file_attributes: u32,
		info: &dokan::OperationInfo<'c, 'h, Self>,
		context: &'c Self::Context,
	) -> dokan::OperationResult<()> {
		Err(STATUS_NOT_IMPLEMENTED)
	}

    fn set_file_time(
		&'h self,
		file_name: &U16CStr,
		creation_time: dokan::FileTimeOperation,
		last_access_time: dokan::FileTimeOperation,
		last_write_time: dokan::FileTimeOperation,
		info: &dokan::OperationInfo<'c, 'h, Self>,
		context: &'c Self::Context,
	) -> dokan::OperationResult<()> {
		Err(STATUS_NOT_IMPLEMENTED)
	}

    fn delete_file(
		&'h self,
		file_name: &U16CStr,
		info: &dokan::OperationInfo<'c, 'h, Self>,
		context: &'c Self::Context,
	) -> dokan::OperationResult<()> {
		Err(STATUS_NOT_IMPLEMENTED)
	}

    fn delete_directory(
		&'h self,
		file_name: &U16CStr,
		info: &dokan::OperationInfo<'c, 'h, Self>,
		context: &'c Self::Context,
	) -> dokan::OperationResult<()> {
		Err(STATUS_NOT_IMPLEMENTED)
	}

    fn move_file(
		&'h self,
		file_name: &U16CStr,
		new_file_name: &U16CStr,
		replace_if_existing: bool,
		info: &dokan::OperationInfo<'c, 'h, Self>,
		context: &'c Self::Context,
	) -> dokan::OperationResult<()> {
		Err(STATUS_NOT_IMPLEMENTED)
	}

    fn set_end_of_file(
		&'h self,
		file_name: &U16CStr,
		offset: i64,
		info: &dokan::OperationInfo<'c, 'h, Self>,
		context: &'c Self::Context,
	) -> dokan::OperationResult<()> {
		Err(STATUS_NOT_IMPLEMENTED)
	}

    fn set_allocation_size(
		&'h self,
		file_name: &U16CStr,
		alloc_size: i64,
		info: &dokan::OperationInfo<'c, 'h, Self>,
		context: &'c Self::Context,
	) -> dokan::OperationResult<()> {
		Err(STATUS_NOT_IMPLEMENTED)
	}

    fn lock_file(
		&'h self,
		file_name: &U16CStr,
		offset: i64,
		length: i64,
		info: &dokan::OperationInfo<'c, 'h, Self>,
		context: &'c Self::Context,
	) -> dokan::OperationResult<()> {
		Err(STATUS_NOT_IMPLEMENTED)
	}

    fn unlock_file(
		&'h self,
		file_name: &U16CStr,
		offset: i64,
		length: i64,
		info: &dokan::OperationInfo<'c, 'h, Self>,
		context: &'c Self::Context,
	) -> dokan::OperationResult<()> {
		Err(STATUS_NOT_IMPLEMENTED)
	}

    fn get_disk_free_space(
		&'h self,
		info: &dokan::OperationInfo<'c, 'h, Self>,
	) -> dokan::OperationResult<dokan::DiskSpaceInfo> {
		Err(STATUS_NOT_IMPLEMENTED)
	}

    fn get_volume_information(
		&'h self,
		info: &dokan::OperationInfo<'c, 'h, Self>,
	) -> dokan::OperationResult<dokan::VolumeInfo> {
		// TODO:磁盘卷信息
		Err(STATUS_NOT_IMPLEMENTED)
	}

    fn mounted(
		&'h self,
		mount_point: &U16CStr,
		info: &dokan::OperationInfo<'c, 'h, Self>,
	) -> dokan::OperationResult<()> {
		Ok(())
	}

    fn unmounted(&'h self, info: &dokan::OperationInfo<'c, 'h, Self>) -> dokan::OperationResult<()> {
		Ok(())
	}

    fn get_file_security(
		&'h self,
		file_name: &U16CStr,
		security_information: u32,
		security_descriptor: PSECURITY_DESCRIPTOR,
		buffer_length: u32,
		info: &dokan::OperationInfo<'c, 'h, Self>,
		context: &'c Self::Context,
	) -> dokan::OperationResult<u32> {
		Err(STATUS_NOT_IMPLEMENTED)
	}

    fn set_file_security(
		&'h self,
		file_name: &U16CStr,
		security_information: u32,
		security_descriptor: winnt::PSECURITY_DESCRIPTOR,
		buffer_length: u32,
		info: &dokan::OperationInfo<'c, 'h, Self>,
		context: &'c Self::Context,
	) -> dokan::OperationResult<()> {
		Err(STATUS_NOT_IMPLEMENTED)
	}

    fn find_streams(
		&'h self,
		file_name: &U16CStr,
		fill_find_stream_data: impl FnMut(&dokan::FindStreamData) -> dokan::FillDataResult,
		info: &dokan::OperationInfo<'c, 'h, Self>,
		context: &'c Self::Context,
	) -> dokan::OperationResult<()> {
		Err(STATUS_NOT_IMPLEMENTED)
	}
}