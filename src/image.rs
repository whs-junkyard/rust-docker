#[derive(RustcEncodable, RustcDecodable)]
#[allow(non_snake_case)]
pub struct Image {
    pub Created: u64,
    pub Id: String,
    pub ParentId: String,
    pub RepoTags: Vec<String>,
    pub Size: u64,
    pub VirtualSize: u64
}

impl Clone for Image {
    fn clone(&self) -> Self {
        let image = Image {
            Created: self.Created,
            Id: self.Id.clone(),
            ParentId: self.ParentId.clone(),
            RepoTags: self.RepoTags.clone(),
            Size: self.Size,
            VirtualSize: self.VirtualSize
        };
        return image;
    }
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct ImageStatus {
    pub status: Option<String>,
    pub error: Option<String>
}

impl Clone for ImageStatus {
    fn clone(&self) -> Self {
        let image_status = ImageStatus {
            status: self.status.clone(),
            error: self.error.clone()
        };
        return image_status;
    }
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
#[allow(non_snake_case)]
pub struct ImageDeleteAction {
    pub Untagged: Option<String>,
    pub Deleted: Option<String>,
}

impl Clone for ImageDeleteAction {
    fn clone(&self) -> Self {
        ImageDeleteAction {
            Untagged: self.Untagged.clone(),
            Deleted: self.Deleted.clone()
        }
    }
}
