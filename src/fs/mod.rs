use crate::common;

pub struct FileSystemDispatcher {
    m_disp: common::Dispatcher,
}

impl FileSystemDispatcher {
    fn new() -> FileSystemDispatcher {
        let mut fs = FileSystemDispatcher {
            m_disp: common::Dispatcher::new()
        };
        fs.m_disp.add_cmd::<ListDirCmd>();
        fs
    }
}

impl common::Command for FileSystemDispatcher {
    fn create() -> Box<FileSystemDispatcher> {
        Box::<>::new(FileSystemDispatcher::new())
    }
    fn name() -> &'static str { "fs" }
    fn fill_subcommand<'a, 'b>(&self, app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
        let fs_sub_cmd = clap::App::new(Self::name());
        let fs_sub_cmd = self.m_disp.fill_subcommands(fs_sub_cmd);
        app.subcommand(fs_sub_cmd)
    }
    fn run(&self, args: Option<&clap::ArgMatches>) -> Result<(), Box<dyn std::error::Error>> {
        let (cmd_name, args) = args.unwrap().subcommand();
        self.m_disp.run(cmd_name, args)
    }
}

struct ListDirCmd;

impl ListDirCmd {
    fn new() -> ListDirCmd { ListDirCmd {} }
}

impl common::Command for ListDirCmd {
    fn create() -> Box<ListDirCmd> {
        Box::<>::new(ListDirCmd::new())
    }
    fn name() -> &'static str { "list_dir" }
    fn fill_subcommand<'a, 'b>(&self, app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
        let sub_cmd =
            clap::App::new(Self::name())
                .arg(
                    clap::Arg::with_name("path")
                        .required(true));
        app.subcommand(sub_cmd)
    }
    fn run(&self, args: Option<&clap::ArgMatches>) -> Result<(), Box<dyn std::error::Error>> {
        let args = args.unwrap();
        let path = args.value_of("path").unwrap();
        let mut explorer = FileExplorer::create(path, false, false)?;

        while !explorer.eof() {
            let file = explorer.next()?;
            let filename = file.m_path.file_name().unwrap().to_str().unwrap();
            //let filename = file.m_path.to_str().unwrap();
            if file.m_meta.is_dir() {
                println!("[{}]", filename);
            } else {
                println!("{}", filename);
            }
        }
        Ok(())
    }
}

struct FileInfo {
    m_path: std::path::PathBuf,
    m_meta: std::fs::Metadata,
}

impl FileInfo {
    fn new(path: &str, meta: std::fs::Metadata) -> FileInfo {
        FileInfo {
            m_path: std::path::PathBuf::from(path),
            m_meta: meta,
        }
    }
    fn from_entry(entry: std::fs::DirEntry) -> FileInfo {
        FileInfo {
            m_path: entry.path(),
            m_meta: entry.metadata().unwrap(),
        }
    }
}

struct FileExplorer {
    m_folders: std::collections::LinkedList<std::path::PathBuf>,
    m_files: std::collections::LinkedList<FileInfo>,
    m_hide_folders: bool,
    m_recursive: bool,
}

impl FileExplorer {
    fn create(path: &str, hide_folders: bool, recursive: bool) -> Result<FileExplorer, std::io::Error> {
        let mut explorer = FileExplorer {
            m_folders: std::collections::LinkedList::new(),
            m_files: std::collections::LinkedList::new(),
            m_hide_folders: hide_folders,
            m_recursive: recursive,
        };

        let attr = std::fs::metadata(path)?;

        if attr.is_dir() {
            explorer.m_folders.push_back(std::path::PathBuf::from(path));
            explorer.load_next()?;
        } else {
            explorer.m_files.push_back(FileInfo::new(path, attr));
        }

        Ok(explorer)
    }
    fn eof(&self) -> bool {
        self.m_folders.is_empty() && self.m_files.is_empty()
    }
    fn next(&mut self) -> Result<FileInfo, std::io::Error> {
        let file = self.m_files.pop_front().unwrap();
        if self.m_files.is_empty() {
            self.load_next()?;
        }
        Ok(file)
    }
    fn load_next(&mut self) -> std::io::Result<()> {
        while !self.m_folders.is_empty() && self.m_files.is_empty() {
            self.load_next_dir()?;
        }
        Ok(())
    }
    fn load_next_dir(&mut self) -> std::io::Result<()> {
        let dir = self.m_folders.pop_front().unwrap();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if self.m_recursive {
                    self.m_folders.push_back(path);
                }
                if !self.m_hide_folders {
                    self.m_files.push_back(FileInfo::from_entry(entry));
                }
            } else {
                self.m_files.push_back(FileInfo::from_entry(entry));
            }
        }
        Ok(())
    }
}