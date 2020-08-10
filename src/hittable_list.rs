/*
class hittable_list : public hittable {
    public:
        hittable_list() {}
        hittable_list(shared_ptr<hittable> object) { add(object); }

        void clear() { objects.clear(); }
        void add(shared_ptr<hittable> object) { objects.push_back(object); }

        virtual bool hit(
            const ray& r, double tmin, double tmax, hit_record& rec) const override;

    public:
        std::vector<shared_ptr<hittable>> objects;
};*/



struct HittableList<T> {
    objects: Vec<&T>,
}

impl HittableList {
    fn new(object: &T) -> HittableList {
        HittableList { objects: vec![] }
    }

    fn clear(&mut self) {
        self.objects = vec![];
    }

    fn add(&mut self, object: T) {
        self.objects.append(object);
    }
}

